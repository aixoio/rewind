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

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

pub fn add_paths(paths: &[String]) -> anyhow::Result<()> {
    let output = Command::new("git").arg("add").args(paths).output()?;

    if !output.status.success() {
        return Err(anyhow!("error: non success exit code from git"));
    }

    Ok(())
}

pub fn commit(message: &str) -> anyhow::Result<()> {
    let output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(message)
        .output()?;

    if !output.status.success() {
        return Err(anyhow!("error: non success exit code from git"));
    }

    Ok(())
}

pub fn init_repo() -> anyhow::Result<()> {
    let output = Command::new("git").arg("init").output()?;

    if !output.status.success() {
        return Err(anyhow!("error: non success exit code from git"));
    }

    Ok(())
}

pub fn diff() -> anyhow::Result<()> {
    let output = Command::new("git")
        .arg("--no-pager")
        .arg("diff")
        .arg("--color=always")
        .output()?;

    if !output.status.success() {
        return Err(anyhow!("error: non success exit code from git"));
    }

    Ok(())
}
