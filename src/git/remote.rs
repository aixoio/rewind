use std::process::{Command, Stdio};

use anyhow::anyhow;

pub fn upstream() -> Option<String> {
    let Ok(output) = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("--symbolic-full-name")
        .arg("@{u}")
        .output()
    else {
        return None;
    };

    let stdout = String::from_utf8_lossy(&output.stdout);

    if !output.status.success() {
        return None;
    }

    Some(stdout.to_string())
}

/// sets the upstream to origin HEAD
///
/// invokes `git push --set-upstream origin HEAD`
pub fn push_set_upstream() -> anyhow::Result<()> {
    let output = Command::new("git")
        .arg("push")
        .arg("--set-upstream")
        .arg("origin")
        .arg("HEAD")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    if !output.status.success() {
        return Err(anyhow!("error: non success exit code from git"));
    }

    Ok(())
}

/// invokes `git push`
pub fn push() -> anyhow::Result<()> {
    let output = Command::new("git")
        .arg("push")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    if !output.status.success() {
        return Err(anyhow!("error: non success exit code from git"));
    }

    Ok(())
}

/// sets the upstream to origin HEAD
///
/// invokes `git pull --set-upstream origin HEAD`
pub fn pull_set_upstream() -> anyhow::Result<()> {
    let output = Command::new("git")
        .arg("pull")
        .arg("--set-upstream")
        .arg("origin")
        .arg("HEAD")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    if !output.status.success() {
        return Err(anyhow!("error: non success exit code from git"));
    }

    Ok(())
}

/// invokes `git pull`
pub fn pull() -> anyhow::Result<()> {
    let output = Command::new("git")
        .arg("pull")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    if !output.status.success() {
        return Err(anyhow!("error: non success exit code from git"));
    }

    Ok(())
}
