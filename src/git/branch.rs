use std::process::{Command, Stdio};

use anyhow::anyhow;

pub fn current_branch() -> anyhow::Result<String> {
    let output = Command::new("git")
        .arg("branch")
        .arg("--show-current")
        .output()?;

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

pub fn all_branches() -> anyhow::Result<String> {
    let output = Command::new("git")
        .arg("--no-pager")
        .arg("branch")
        .arg("--list")
        .arg("--format=%(refname:short)")
        .output()?;

    Ok(String::from_utf8(output.stdout)?)
}

/// wraps `git show-ref --verify --quiet refs/heads/{branch}`
pub fn branch_exists(branch: &str) -> bool {
    let Ok(output) = Command::new("git")
        .arg("show-ref")
        .arg("--verify")
        .arg("--quiet")
        .arg(format!("refs/heads/{branch}"))
        .stderr(Stdio::null())
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .status()
    else {
        return false;
    };

    output.success()
}

pub fn create_branch(branch: &str) -> anyhow::Result<()> {
    let output = Command::new("git").arg("branch").arg(branch).output()?;

    if !output.status.success() {
        return Err(anyhow!(
            "error: git: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}

pub fn delete_branch(branch: &str) -> anyhow::Result<()> {
    let output = Command::new("git")
        .arg("branch")
        .arg("-d")
        .arg(branch.trim())
        .output()?;

    if !output.status.success() {
        return Err(anyhow!(
            "error: git: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}

pub fn delete_remote_branch(branch: &str) -> anyhow::Result<()> {
    let output = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("--delete")
        .arg(branch.trim())
        .output()?;

    if !output.status.success() {
        return Err(anyhow!(
            "error: git: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}

/// must be used with `git --no-pager branch --list --format='%(refname:short)'`
pub fn parse_branch_names(stdout: &str) -> impl Iterator<Item = &str> {
    stdout.trim().lines().filter_map(|line| {
        if line.trim().is_empty() {
            return None;
        }

        Some(line.trim())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_branch_names() {
        let input = "master\nmain\nstaging";
        let target = vec!["master", "main", "staging"];

        assert_eq!(target, parse_branch_names(input).collect::<Vec<_>>())
    }

    #[test]
    fn test_parse_branch_names_empty() {
        let input = "\n\t\t\n";
        let target: Vec<&str> = vec![];

        assert_eq!(target, parse_branch_names(input).collect::<Vec<_>>())
    }
}
