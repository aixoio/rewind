use std::process::Command;

pub fn current_branch() -> anyhow::Result<String> {
    let output = Command::new("git")
        .arg("branch")
        .arg("--show-current")
        .output()?;

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

pub fn all_branches() -> anyhow::Result<Vec<String>> {
    let output = Command::new("git")
        .arg("--no-pager")
        .arg("branch")
        .arg("--list")
        .arg("--format=%(refname:short)")
        .output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);

    Ok(parse_branch_names(&stdout)
        .iter()
        .map(|b| b.to_string())
        .collect())
}

/// must be used with `git --no-pager branch --list --format='%(refname:short)'`
fn parse_branch_names(stdout: &str) -> Vec<&str> {
    stdout
        .trim()
        .lines()
        .filter_map(|line| {
            if line.trim().is_empty() {
                return None;
            }

            Some(line.trim())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_branch_names() {
        let input = "master\nmain\nstaging";
        let target = vec!["master", "main", "staging"];

        assert_eq!(target, parse_branch_names(input))
    }

    #[test]
    fn test_parse_branch_names_empty() {
        let input = "\n\t\t\n";
        let target: Vec<&str> = vec![];

        assert_eq!(target, parse_branch_names(input))
    }
}
