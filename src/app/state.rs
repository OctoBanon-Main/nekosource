use super::error::NekoError;
use super::language::{Language, Locale};
use std::sync::mpsc;

pub enum WorkerMessage {
    Success(String),
    Error(NekoError),
    CloneSuccess {
        local_path: String,
        remote_url: String,
    },

    NeedPassphrase {
        sender: mpsc::SyncSender<Option<String>>,
        key_path: String,
    },
}

pub enum Status {
    Idle,
    Loading(String),
    Success(String),
    Error(String),
}

pub struct NekoApp {
    pub repos: Vec<(String, String)>,
    pub selected_idx: Option<usize>,

    pub repo_branches: Vec<super::git::BranchInfo>,

    pub selected_commits: Vec<super::git::CommitInfo>,
    pub selected_changes: Vec<String>,
    pub log_loaded_for: Option<usize>,

    pub current_lang: Language,
    pub locale: Locale,

    pub status: Status,
    pub show_settings: bool,
    pub show_about: bool,

    pub clone_url_input: String,
    pub clone_dest_input: String,
    pub clone_branch_input: String,

    pub commit_msg_input: String,

    pub username: String,
    pub email: String,
    pub enable_gpg: bool,
    pub gpg_key_id: String,
    pub gpg_keys: Vec<(String, String)>,

    pub worker_rx: Option<mpsc::Receiver<WorkerMessage>>,

    pub passphrase_input: String,
    pub passphrase_sender: Option<mpsc::SyncSender<Option<String>>>,
    pub passphrase_key_path: String,
    pub show_passphrase_dialog: bool,
}

impl NekoApp {
    pub fn new() -> Self {
        let default_lang = Language::En;

        let (username, email) = super::git::read_user_config();

        let gpg_keys = super::gpg::list_keys();

        let gpg_key_id = {
            use git2::Config;
            Config::open_default()
                .ok()
                .and_then(|c| c.get_string("user.signingkey").ok())
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| {
                    gpg_keys
                        .first()
                        .map(|(id, _)| id.clone())
                        .unwrap_or_default()
                })
        };

        let repos = super::git::load_repos();
        let repo_branches = repos
            .iter()
            .map(|(path, _)| super::git::load_branch(path))
            .collect();

        Self {
            repos,
            selected_idx: None,
            repo_branches,
            selected_commits: vec![],
            selected_changes: vec![],
            log_loaded_for: None,
            current_lang: default_lang,
            locale: Locale::new(default_lang),
            status: Status::Idle,
            show_settings: false,
            show_about: false,
            clone_url_input: String::new(),
            clone_dest_input: "repos".into(),
            clone_branch_input: String::new(),
            commit_msg_input: String::new(),
            username,
            email,
            enable_gpg: false,
            gpg_key_id,
            gpg_keys,
            worker_rx: None,
            passphrase_input: String::new(),
            passphrase_sender: None,
            passphrase_key_path: String::new(),
            show_passphrase_dialog: false,
        }
    }

    #[inline]
    pub fn t(&self, key: &str) -> String {
        self.locale.t(key)
    }

    pub fn is_busy(&self) -> bool {
        matches!(self.status, Status::Loading(_))
    }

    pub fn save_repos(&self) {
        super::git::save_repos(&self.repos);
    }

    pub fn refresh_all_branches(&mut self) {
        self.repo_branches = self
            .repos
            .iter()
            .map(|(path, _)| super::git::load_branch(path))
            .collect();
    }

    pub fn refresh_selected_log(&mut self) {
        self.log_loaded_for = self.selected_idx;
        self.selected_commits = match self.selected_idx {
            Some(idx) => super::git::load_commits(&self.repos[idx].0, 25),
            None => vec![],
        };
        self.selected_changes = match self.selected_idx {
            Some(idx) => super::git::load_changed_files(&self.repos[idx].0),
            None => vec![],
        };
    }
}
