use super::super::error::{NekoError, NekoResult};
use super::credentials::{build_ssh_command, prepare_ssh_key};
use std::path::Path;
use std::process::Command;

pub fn clone_repo(
    url: &str,
    dest: &str,
    branch: &str,
    request_passphrase: impl Fn(String) -> Option<String>,
) -> NekoResult<String> {
    let dest_path = Path::new(dest);
    if dest_path.exists() {
        return Err(NekoError::GitDestExists(dest.to_string()));
    }

    let key_guard = if is_ssh_url(url) {
        prepare_ssh_key(&request_passphrase)?
    } else {
        None
    };

    let mut cmd = Command::new("git");
    cmd.arg("clone").arg("--progress");

    if !branch.is_empty() {
        cmd.arg("--branch").arg(branch);
    }

    cmd.arg(url).arg(dest).env("GIT_TERMINAL_PROMPT", "0");

    if let Some(ref key) = key_guard {
        cmd.env("GIT_SSH_COMMAND", build_ssh_command(key.path()));
    }

    let output = cmd
        .output()
        .map_err(|e| NekoError::GitClone(e.to_string()))?;

    drop(key_guard);

    if output.status.success() {
        Ok(dest.to_string())
    } else {
        let msg = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(NekoError::GitClone(msg))
    }
}

pub fn repo_name_from_url(url: &str) -> String {
    let name = url
        .trim()
        .split(['?', '#'])
        .next()
        .unwrap_or_default()
        .trim_end_matches('/')
        .rsplit(['/', ':'])
        .next()
        .unwrap_or("repo")
        .trim_end_matches(".git");

    if name.is_empty() {
        "repo".into()
    } else {
        name.into()
    }
}

fn is_ssh_url(url: &str) -> bool {
    url.starts_with("ssh://") || (!url.starts_with("http") && url.contains('@'))
}