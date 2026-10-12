use std::io::Write;
use std::process::{Command, Stdio};

use super::error::{NekoError, NekoResult};

pub fn sign(data: &str, key_id: &str) -> NekoResult<String> {
    let mut child = Command::new("gpg")
        .args([
            "--status-fd",
            "2",
            "--bsign",
            "--local-user",
            key_id,
            "--armor",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| NekoError::GpgLaunch(e.to_string()))?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(data.as_bytes())
            .map_err(|e| NekoError::GpgStdinWrite(e.to_string()))?;
    }

    let output = child
        .wait_with_output()
        .map_err(|e| NekoError::GpgWait(e.to_string()))?;

    if output.status.success() {
        String::from_utf8(output.stdout).map_err(|e| NekoError::GpgOutputEncoding(e.to_string()))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        Err(NekoError::GpgFailure(stderr))
    }
}

pub fn list_keys() -> Vec<(String, String)> {
    let output = match Command::new("gpg")
        .args(["--list-secret-keys", "--with-colons", "--keyid-format=long"])
        .output()
    {
        Ok(o) => o,
        Err(_) => return vec![],
    };

    let text = String::from_utf8_lossy(&output.stdout);
    let mut keys: Vec<(String, String)> = Vec::new();
    let mut current_id = String::new();

    for line in text.lines() {
        let fields: Vec<&str> = line.splitn(11, ':').collect();
        if fields.len() < 2 {
            continue;
        }

        match fields[0] {
            "sec" if fields.len() > 4 => {
                current_id = fields[4].to_string();
            }
            "uid" if !current_id.is_empty() => {
                let uid = fields.get(9).copied().unwrap_or("").trim();
                let label = if uid.is_empty() {
                    current_id.clone()
                } else {
                    format!("{} — {}", current_id, uid)
                };
                keys.push((current_id.clone(), label));
                current_id.clear();
            }
            _ => {}
        }
    }

    keys
}
