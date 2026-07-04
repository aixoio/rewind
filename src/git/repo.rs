use std::{
    ffi::OsStr,
    path::Path,
    process::{Command, Stdio},
};

use anyhow::anyhow;

pub fn is_git_repo() -> bool {
    Path::new(".git").exists()
}

pub fn add_paths<S: AsRef<str>>(paths: &[S]) -> anyhow::Result<()> {
    let output = Command::new("git")
        .arg("add")
        .args(paths.iter().map(|p| p.as_ref()))
        .output()?;

    if !output.status.success() {
        return Err(anyhow!(
            "error: git: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
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
        return Err(anyhow!(
            "error: git: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}

pub fn init_repo() -> anyhow::Result<()> {
    let output = Command::new("git").arg("init").output()?;

    if !output.status.success() {
        return Err(anyhow!(
            "error: git: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
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
        return Err(anyhow!(
            "error: git: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}

pub fn checkout(target: &str) -> anyhow::Result<()> {
    let output = Command::new("git").arg("checkout").arg(target).output()?;

    if !output.status.success() {
        return Err(anyhow!(
            "error: git: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}

pub fn merge(source: &str) -> anyhow::Result<()> {
    let output = Command::new("git").arg("merge").arg(source).output()?;

    if !output.status.success() {
        return Err(anyhow!(
            "error: git: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}

pub fn reset() -> anyhow::Result<()> {
    let output = Command::new("git").arg("reset").arg("--hard").output()?;

    if !output.status.success() {
        return Err(anyhow!(
            "error: git: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}
