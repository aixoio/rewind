use std::{path::Path, process::Command};

use anyhow::Ok;

pub fn is_git_repo() -> bool {
    Path::new(".git").exists()
}

pub fn current_branch() -> anyhow::Result<String> {
    let output = Command::new("git")
        .arg("branch")
        .arg("--show-current")
        .output()?;

    Ok(String::from_utf8_lossy(&output.stdout)
        .to_string()
        .trim()
        .to_string())
}
