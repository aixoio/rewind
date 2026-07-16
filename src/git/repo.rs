use std::{
    ffi::OsStr,
    process::{Command, Stdio},
};

use anyhow::anyhow;

pub fn is_git_repo() -> bool {
    let Ok(output) = Command::new("git")
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .stderr(Stdio::null())
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .status()
    else {
        return false;
    };

    output.success()
}

pub fn add_paths<S: AsRef<str>>(paths: &[S]) -> anyhow::Result<()> {
    let output = Command::new("git")
        .arg("add")
        .args(paths.iter().map(|p| p.as_ref()))
        .output()?;

    if !output.status.success() {
        return Err(anyhow!("git: {}", String::from_utf8_lossy(&output.stderr)));
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
        return Err(anyhow!("git: {}", String::from_utf8_lossy(&output.stderr)));
    }

    Ok(())
}

pub fn init_repo() -> anyhow::Result<()> {
    let output = Command::new("git").arg("init").output()?;

    if !output.status.success() {
        return Err(anyhow!("git: {}", String::from_utf8_lossy(&output.stderr)));
    }

    Ok(())
}

pub fn diff_stat() -> anyhow::Result<()> {
    diff(["--stat"])
}

pub fn diff<I, S>(args: I) -> anyhow::Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = Command::new("git")
        .arg("--no-pager")
        .arg("diff")
        .arg("--color=always")
        .args(args)
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stdin(Stdio::inherit())
        .output()?;

    if !output.status.success() {
        return Err(anyhow!("git: {}", String::from_utf8_lossy(&output.stderr)));
    }

    Ok(())
}

pub fn checkout(target: &str) -> anyhow::Result<()> {
    let output = Command::new("git").arg("checkout").arg(target).output()?;

    if !output.status.success() {
        return Err(anyhow!("git: {}", String::from_utf8_lossy(&output.stderr)));
    }

    Ok(())
}

pub fn merge(source: &str) -> anyhow::Result<()> {
    let output = Command::new("git").arg("merge").arg(source).output()?;

    if !output.status.success() {
        return Err(anyhow!("git: {}", String::from_utf8_lossy(&output.stderr)));
    }

    Ok(())
}

pub fn reset() -> anyhow::Result<()> {
    let output = Command::new("git").arg("reset").arg("--hard").output()?;

    if !output.status.success() {
        return Err(anyhow!("git: {}", String::from_utf8_lossy(&output.stderr)));
    }

    Ok(())
}

pub fn revert(hash: &str) -> anyhow::Result<()> {
    let output = Command::new("git")
        .arg("revert")
        .arg("--no-commit")
        .arg(hash)
        .output()?;

    if !output.status.success() {
        return Err(anyhow!("git: {}", String::from_utf8_lossy(&output.stderr)));
    }

    Ok(())
}
