use std::{
    path::Path,
    process::{Command, Stdio},
};

use anyhow::anyhow;

pub fn is_git_repo() -> bool {
    Path::new(".git").exists()
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
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stdin(Stdio::inherit())
        .output()?;

    if !output.status.success() {
        return Err(anyhow!("error: non success exit code from git"));
    }

    Ok(())
}

pub fn checkout(target: &str) -> anyhow::Result<()> {
    let output = Command::new("git").arg("checkout").arg(target).output()?;

    if !output.status.success() {
        return Err(anyhow!("error: non success exit code from git"));
    }

    Ok(())
}
