use std::{path::Path, process::Command};

use anyhow::anyhow;

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

pub fn add_files(path: &String) -> anyhow::Result<()> {
    let output = Command::new("git").arg("add").arg(path).output()?;

    if !output.status.success() {
        return Err(anyhow!("error: non success exit code from git"));
    }

    Ok(())
}
