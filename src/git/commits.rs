use std::process::Command;

use anyhow::anyhow;

use crate::getter;

#[derive(Debug, PartialEq, Eq)]
pub struct CommitInfo<'a> {
    hash: &'a str,
    message: &'a str,
    author: &'a str,
    date: &'a str,
}

impl<'a> CommitInfo<'a> {
    getter!(hash, &'a str);
    getter!(message, &'a str);
    getter!(author, &'a str);
    getter!(date, &'a str);

    fn build(
        hash: &'a str,
        message: &'a str,
        author: &'a str,
        date: &'a str,
    ) -> anyhow::Result<CommitInfo<'a>> {
        if hash.trim().is_empty() {
            return Err(anyhow!("missing hash"));
        }

        if message.trim().is_empty() {
            return Err(anyhow!("missing message"));
        }

        if author.trim().is_empty() {
            return Err(anyhow!("missing author"));
        }

        if date.trim().is_empty() {
            return Err(anyhow!("missing date"));
        }

        Ok(CommitInfo {
            hash,
            message,
            author,
            date,
        })
    }
}

pub fn fetch_commit_info(hash: &str) -> anyhow::Result<String> {
    let output = Command::new("git")
        .arg("show")
        .arg("--no-patch")
        .arg("--format=%H%x1f%s%x1f%an%x1f%ad")
        .arg("--date=short")
        .arg(hash)
        .output()?;

    if !output.status.success() {
        return Err(anyhow!("git: {}", String::from_utf8_lossy(&output.stderr)));
    }

    let stdout = String::from_utf8(output.stdout)?;

    Ok(stdout)
}

/// input must come from `git show --no-patch --format=%H%x1f%s%x1f%an%x1f%ad --date=short [hash]`
pub fn parse_commit_info<'a>(input: &'a str) -> Option<CommitInfo<'a>> {
    let mut data = input.trim().split('\x1f');

    let hash = data.next()?;
    let message = data.next()?;
    let author = data.next()?;
    let date = data.next()?;

    let Ok(commit) = CommitInfo::build(hash, message, author, date) else {
        return None;
    };

    Some(commit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_commit_info_parses_valid_git_show_output() {
        let input = "abc123\x1fInitial commit\x1fJane Doe\x1f2026-07-04";

        let info = parse_commit_info(input).unwrap();

        assert_eq!(info.hash(), &"abc123");
        assert_eq!(info.message(), &"Initial commit");
        assert_eq!(info.author(), &"Jane Doe");
        assert_eq!(info.date(), &"2026-07-04");
    }

    #[test]
    fn parse_commit_info_trims_surrounding_whitespace() {
        let input = "\nabc123\x1fFix bug\x1fJohn Smith\x1f2026-01-15\n";

        let info = parse_commit_info(input).unwrap();

        assert_eq!(
            info,
            CommitInfo {
                hash: "abc123",
                message: "Fix bug",
                author: "John Smith",
                date: "2026-01-15",
            }
        );
    }

    #[test]
    fn parse_commit_info_returns_none_when_missing_fields() {
        let input = "abc123\x1fOnly message\x1fJane Doe";

        let info = parse_commit_info(input);

        assert_eq!(info, None);
    }

    #[test]
    fn parse_commit_info_allows_extra_fields() {
        let input = "abc123\x1fInitial commit\x1fJane Doe\x1f2026-07-04\x1fextra";

        let info = parse_commit_info(input).unwrap();

        assert_eq!(
            info,
            CommitInfo {
                hash: "abc123",
                message: "Initial commit",
                author: "Jane Doe",
                date: "2026-07-04",
            }
        );
    }

    #[test]
    fn parse_commit_info_allows_empty_fields() {
        let input = "\x1f\x1f\x1f";

        let info = parse_commit_info(input);

        assert!(info.is_none());
    }
}
