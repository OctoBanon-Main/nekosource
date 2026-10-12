use super::super::{
    git,
    state::{NekoApp, Status, WorkerMessage},
};
use std::{path::Path, sync::mpsc};

pub fn poll_worker(app: &mut NekoApp) -> bool {
    let msg = app.worker_rx.as_ref().and_then(|rx| rx.try_recv().ok());
    let Some(msg) = msg else { return false };

    match msg {
        WorkerMessage::Success(s) => {
            app.status = Status::Success(s);
            app.worker_rx = None;
            app.log_loaded_for = None;
        }

        WorkerMessage::Error(e) => {
            app.status = Status::Error(e.localize(&app.locale));
            app.worker_rx = None;
        }

        WorkerMessage::CloneSuccess {
            local_path,
            remote_url,
        } => {
            let already_exists = app.repos.iter().any(|(p, _)| p == &local_path);
            if !already_exists {
                app.repos.push((local_path.clone(), remote_url.clone()));
            }
            app.status = Status::Success(format!("{} → {}", remote_url, local_path));
            app.clone_url_input.clear();
            app.clone_branch_input.clear();
            app.worker_rx = None;
            app.refresh_all_branches();
            app.save_repos();
        }

        WorkerMessage::NeedPassphrase { sender, key_path } => {
            app.passphrase_sender = Some(sender);
            app.passphrase_key_path = key_path;
            app.show_passphrase_dialog = true;
        }
    }
    true
}

pub fn spawn_clone(app: &mut NekoApp) {
    let url = app.clone_url_input.trim().to_string();
    if url.is_empty() {
        return;
    }

    let branch = app.clone_branch_input.trim().to_string();
    let name = git::repo_name_from_url(&url);
    let dest = Path::new(app.clone_dest_input.trim())
        .join(name)
        .to_string_lossy()
        .into_owned();
    let remote = url.clone();

    let (tx, rx) = mpsc::channel::<WorkerMessage>();
    app.worker_rx = Some(rx);
    app.status = Status::Loading(app.t("status-cloning"));

    let tx_pp = tx.clone();

    std::thread::spawn(move || {
        let request_passphrase = move |key_path: String| -> Option<String> {
            let (pp_tx, pp_rx) = mpsc::sync_channel::<Option<String>>(1);
            tx_pp
                .send(WorkerMessage::NeedPassphrase {
                    sender: pp_tx,
                    key_path,
                })
                .ok();
            pp_rx.recv().ok().flatten()
        };

        let result = git::clone_repo(&url, &dest, &branch, request_passphrase);
        tx.send(match result {
            Ok(path) => WorkerMessage::CloneSuccess {
                local_path: path,
                remote_url: remote,
            },
            Err(e) => WorkerMessage::Error(e),
        })
        .ok();
    });
}

pub fn spawn_commit(app: &mut NekoApp) {
    let Some(idx) = app.selected_idx else { return };
    let repo_path = app.repos[idx].0.clone();

    let cfg = git::CommitConfig {
        username: app.username.clone(),
        email: app.email.clone(),
        message: app.commit_msg_input.clone(),
        gpg_key_id: if app.enable_gpg {
            Some(app.gpg_key_id.clone())
        } else {
            None
        },
    };

    let (tx, rx) = mpsc::channel::<WorkerMessage>();
    app.worker_rx = Some(rx);
    app.status = Status::Loading(app.t("status-committing"));
    app.commit_msg_input.clear();

    let tx_pp = tx.clone();

    std::thread::spawn(move || {
        let request_passphrase = move |key_path: String| -> Option<String> {
            let (pp_tx, pp_rx) = mpsc::sync_channel::<Option<String>>(1);
            tx_pp
                .send(WorkerMessage::NeedPassphrase {
                    sender: pp_tx,
                    key_path,
                })
                .ok();
            pp_rx.recv().ok().flatten()
        };

        let path_clone = repo_path.clone();
        let result = git::stage_commit_push(&repo_path, &cfg, request_passphrase);
        tx.send(match result {
            Ok(_) => WorkerMessage::Success(format!("Push: {}", path_clone)),
            Err(e) => WorkerMessage::Error(e),
        })
        .ok();
    });
}
