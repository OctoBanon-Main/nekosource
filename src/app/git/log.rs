use chrono::{DateTime, Duration, Local, TimeZone};
use git2::{BranchType, Repository, StatusOptions};

#[derive(PartialEq, Eq)]
pub struct CommitInfo {
    pub short_hash: String,
    pub title: String,
    pub author: String,
    pub time_ago: String,
    pub datetime: String,
}

#[derive(PartialEq, Eq)]
pub struct BranchInfo {
    pub name: String,
    pub divergence: Option<(usize, usize)>,
}

pub fn load_branch(repo_path: &str) -> BranchInfo {
    let Ok(repo) = Repository::open(repo_path) else {
        return BranchInfo {
            name: "?".into(),
            divergence: None,
        };
    };
    let Ok(head) = repo.head() else {
        return BranchInfo {
            name: "?".into(),
            divergence: None,
        };
    };
    let Some(name) = head.shorthand() else {
        return BranchInfo {
            name: "?".into(),
            divergence: None,
        };
    };

    let divergence = repo
        .find_branch(name, BranchType::Local)
        .ok()
        .and_then(|branch| {
            let upstream = branch.upstream().ok()?;
            repo.graph_ahead_behind(branch.get().target()?, upstream.get().target()?)
                .ok()
        });

    BranchInfo {
        name: name.into(),
        divergence,
    }
}

pub fn load_changed_files(repo_path: &str) -> Vec<String> {
    let Ok(repo) = Repository::open(repo_path) else {
        return vec![];
    };
    let mut options = StatusOptions::new();
    options.include_untracked(true).recurse_untracked_dirs(true);

    let mut files = repo
        .statuses(Some(&mut options))
        .map(|statuses| {
            statuses
                .iter()
                .filter_map(|entry| entry.path().map(str::to_string))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    files.sort();
    files
}

pub fn load_commits(repo_path: &str, count: usize) -> Vec<CommitInfo> {
    let repo = match Repository::open(repo_path) {
        Ok(r) => r,
        Err(_) => return vec![],
    };

    let mut walk = match repo.revwalk() {
        Ok(w) => w,
        Err(_) => return vec![],
    };

    if walk.push_head().is_err() {
        return vec![];
    }
    walk.set_sorting(git2::Sort::TIME).ok();

    let now = Local::now();

    walk.take(count)
        .filter_map(|oid| {
            let oid = oid.ok()?;
            let commit = repo.find_commit(oid).ok()?;

            let git_time = commit.time();
            let offset = chrono::FixedOffset::east_opt(git_time.offset_minutes() * 60)?;
            let committed: DateTime<chrono::FixedOffset> =
                offset.timestamp_opt(git_time.seconds(), 0).single()?;
            let committed_local = committed.with_timezone(&Local);

            Some(CommitInfo {
                short_hash: format!("{:.7}", oid),
                title: commit.summary().unwrap_or("").to_string(),
                author: commit.author().name().unwrap_or("?").to_string(),
                time_ago: fmt_time_ago(now - committed_local),
                datetime: committed_local.format("%Y-%m-%d %H:%M:%S").to_string(),
            })
        })
        .collect()
}

fn fmt_time_ago(delta: Duration) -> String {
    let secs = delta.num_seconds().max(0);
    let mins = delta.num_minutes().max(0);
    let hours = delta.num_hours().max(0);
    let days = delta.num_days().max(0);
    let weeks = days / 7;

    match secs {
        s if s < 60 => "just now".into(),
        _ if mins < 60 => format!("{}m ago", mins),
        _ if hours < 24 => format!("{}h ago", hours),
        _ if days < 7 => format!("{}d ago", days),
        _ if weeks < 52 => format!("{}w ago", weeks),
        _ => format!("{}y ago", days / 365),
    }
}