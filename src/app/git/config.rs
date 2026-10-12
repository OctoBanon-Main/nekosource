use super::super::error::{NekoError, NekoResult};
use git2::Config;
use std::path::{Path, PathBuf};

pub fn read_user_config() -> (String, String) {
    let Ok(cfg) = Config::open_default().or_else(|_| Config::new()) else {
        return (String::new(), String::new());
    };
    let name = cfg.get_string("user.name").unwrap_or_default();
    let email = cfg.get_string("user.email").unwrap_or_default();
    (name, email)
}

pub fn write_user_config(name: &str, email: &str) -> NekoResult<()> {
    let path = Config::find_global()
        .or_else(|_| Config::find_xdg())
        .map_err(|e| NekoError::GitConfigNotFound(e.message().to_string()))?;

    let mut cfg =
        Config::open(&path).map_err(|e| NekoError::GitConfigOpen(e.message().to_string()))?;

    cfg.set_str("user.name", name)
        .map_err(|e| NekoError::GitConfigWriteName(e.message().to_string()))?;

    cfg.set_str("user.email", email)
        .map_err(|e| NekoError::GitConfigWriteEmail(e.message().to_string()))?;

    Ok(())
}

fn repos_file_path() -> PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    Path::new(&home)
        .join(".config")
        .join("nekosource")
        .join("repos.txt")
}

pub fn load_repos() -> Vec<(String, String)> {
    let path = repos_file_path();
    let content = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return vec![],
    };

    content
        .lines()
        .filter_map(|line| {
            let mut parts = line.splitn(2, '\t');
            let repo_path = parts.next()?.trim();
            let url = parts.next().unwrap_or("").trim();
            if repo_path.is_empty() {
                return None;
            }
            if Path::new(repo_path).exists() {
                Some((repo_path.to_string(), url.to_string()))
            } else {
                None
            }
        })
        .collect()
}

pub fn save_repos(repos: &[(String, String)]) {
    let path = repos_file_path();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let content: String = repos
        .iter()
        .map(|(p, u)| format!("{}\t{}", p, u))
        .collect::<Vec<_>>()
        .join("\n");
    let _ = std::fs::write(&path, content);
}
