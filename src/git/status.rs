use std::process::Command;

use anyhow::Ok;

#[derive(Debug, PartialEq, Eq)]
pub struct StatusResult {
    staged: Vec<String>,
    unstaged: Vec<String>,
    untracked: Vec<String>,
}

macro_rules! getter {
    ($name:ident, $type:ty) => {
        pub fn $name(&self) -> &$type {
            &self.$name
        }
    };
}

impl StatusResult {
    getter!(staged, Vec<String>);
    getter!(unstaged, Vec<String>);
    getter!(untracked, Vec<String>);

    pub fn total_files(&self) -> usize {
        self.staged.len() + self.unstaged.len() + self.untracked.len()
    }
}

impl Default for StatusResult {
    fn default() -> Self {
        Self {
            staged: vec![],
            unstaged: vec![],
            untracked: vec![],
        }
    }
}

pub fn fetch_status() -> anyhow::Result<StatusResult> {
    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);

    Ok(parse_status(&stdout))
}

pub fn fetch_status_raw() -> anyhow::Result<String> {
    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);

    Ok(stdout.to_string())
}

fn parse_status(status: &str) -> StatusResult {
    let mut status_result = StatusResult::default();

    for line in status.lines() {
        let file_name: String = line.chars().skip(2).collect();
        let file_name = file_name.trim().to_owned();

        let (x, y) = (
            line.chars().nth(0).unwrap_or(' '),
            line.chars().nth(1).unwrap_or(' '),
        );

        if x == '?' && y == '?' {
            status_result.untracked.push(file_name);
            continue;
        }

        if x != ' ' {
            status_result.staged.push(file_name.clone());
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

    fn strings(items: Vec<&str>) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_parse_status_basic() {
        let input = vec![" M src/main.rs", "MM src/lib.rs", "D  README.md"].join("\n");

        let target = StatusResult {
            staged: strings(vec!["src/lib.rs", "README.md"]),
            unstaged: strings(vec!["src/main.rs", "src/lib.rs"]),
            ..Default::default()
        };

        assert_eq!(target, parse_status(&input));
    }

    #[test]
    fn test_parse_status_untracked() {
        let input = vec!["?? notes.txt", "?? src/new_file.rs"].join("\n");

        let target = StatusResult {
            untracked: strings(vec!["notes.txt", "src/new_file.rs"]),
            ..Default::default()
        };

        assert_eq!(target, parse_status(&input));
    }

    #[test]
    fn test_parse_status_staged_only() {
        let input = vec!["M  src/main.rs", "A  src/lib.rs", "D  old.txt"].join("\n");

        let target = StatusResult {
            staged: strings(vec!["src/main.rs", "src/lib.rs", "old.txt"]),
            ..Default::default()
        };

        assert_eq!(target, parse_status(&input));
    }

    #[test]
    fn test_parse_status_unstaged_only() {
        let input = vec![" M src/main.rs", " D old.txt"].join("\n");

        let target = StatusResult {
            unstaged: strings(vec!["src/main.rs", "old.txt"]),
            ..Default::default()
        };

        assert_eq!(target, parse_status(&input));
    }

    #[test]
    fn test_parse_status_mixed_staged_and_unstaged() {
        let input = vec!["MM src/lib.rs", "AM src/new.rs", "MD src/delete_later.rs"].join("\n");

        let target = StatusResult {
            staged: strings(vec!["src/lib.rs", "src/new.rs", "src/delete_later.rs"]),
            unstaged: strings(vec!["src/lib.rs", "src/new.rs", "src/delete_later.rs"]),
            ..Default::default()
        };

        assert_eq!(target, parse_status(&input));
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
            staged: strings(vec!["old_name.rs -> new_name.rs"]),
            ..Default::default()
        };

        assert_eq!(target, parse_status(input));
    }
}
