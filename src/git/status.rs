use std::process::Command;

use anyhow::anyhow;

use crate::getter;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct StatusResult<'a> {
    staged: Vec<&'a str>,
    unstaged: Vec<&'a str>,
    untracked: Vec<&'a str>,
}

impl<'a> StatusResult<'a> {
    getter!(staged, Vec<&'a str>);
    getter!(unstaged, Vec<&'a str>);
    getter!(untracked, Vec<&'a str>);

    pub fn total_files(&self) -> usize {
        self.staged.len() + self.unstaged.len() + self.untracked.len()
    }
}

pub fn fetch_status() -> anyhow::Result<String> {
    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .output()?;
    let stdout = String::from_utf8(output.stdout)?;

    if !output.status.success() {
        return Err(anyhow!("git: {}", String::from_utf8_lossy(&output.stderr)));
    }

    Ok(stdout)
}

pub fn parse_status<'a>(status: &'a str) -> StatusResult<'a> {
    let mut status_result = StatusResult::default();

    for line in status.lines() {
        let Some(file_name) = line.get(2..) else {
            continue;
        };
        let file_name = file_name.trim();

        let mut chars = line.chars();
        let x = chars.next().unwrap_or(' ');
        let y = chars.next().unwrap_or(' ');

        if x == '?' && y == '?' {
            status_result.untracked.push(file_name);
            continue;
        }

        if x != ' ' {
            status_result.staged.push(file_name);
        }

        if y != ' ' {
            status_result.unstaged.push(file_name);
        }
    }

    status_result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_status_basic() {
        let input = " M src/main.rs\nMM src/lib.rs\nD  README.md";

        let target = StatusResult {
            staged: vec!["src/lib.rs", "README.md"],
            unstaged: vec!["src/main.rs", "src/lib.rs"],
            ..Default::default()
        };

        assert_eq!(target, parse_status(input));
    }

    #[test]
    fn test_parse_status_untracked() {
        let input = "?? notes.txt\n?? src/new_file.rs";

        let target = StatusResult {
            untracked: vec!["notes.txt", "src/new_file.rs"],
            ..Default::default()
        };

        assert_eq!(target, parse_status(input));
    }

    #[test]
    fn test_parse_status_staged_only() {
        let input = "M  src/main.rs\nA  src/lib.rs\nD  old.txt";

        let target = StatusResult {
            staged: vec!["src/main.rs", "src/lib.rs", "old.txt"],
            ..Default::default()
        };

        assert_eq!(target, parse_status(input));
    }

    #[test]
    fn test_parse_status_unstaged_only() {
        let input = " M src/main.rs\n D old.txt";

        let target = StatusResult {
            unstaged: vec!["src/main.rs", "old.txt"],
            ..Default::default()
        };

        assert_eq!(target, parse_status(input));
    }

    #[test]
    fn test_parse_status_mixed_staged_and_unstaged() {
        let input = "MM src/lib.rs\nAM src/new.rs\nMD src/delete_later.rs";

        let target = StatusResult {
            staged: vec!["src/lib.rs", "src/new.rs", "src/delete_later.rs"],
            unstaged: vec!["src/lib.rs", "src/new.rs", "src/delete_later.rs"],
            ..Default::default()
        };

        assert_eq!(target, parse_status(input));
    }

    #[test]
    fn test_parse_status_empty_input() {
        let input = "";

        let target = StatusResult::default();

        assert_eq!(target, parse_status(input));
    }

    #[test]
    fn test_parse_status_ignored_clean_lines() {
        let input = "\n\n";

        let target = StatusResult::default();

        assert_eq!(target, parse_status(input));
    }

    #[test]
    fn test_parse_status_renamed_file() {
        let input = "R  old_name.rs -> new_name.rs";

        let target = StatusResult {
            staged: vec!["old_name.rs -> new_name.rs"],
            ..Default::default()
        };

        assert_eq!(target, parse_status(input));
    }
}
