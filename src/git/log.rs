use std::process::Command;

use anyhow::anyhow;

#[derive(Debug, PartialEq, Eq)]
pub struct Commit<'a> {
    hash: &'a str,
    date: &'a str,
    refs: Vec<&'a str>,
    subject: &'a str,
}

macro_rules! getter {
    ($name:ident, $type:ty) => {
        pub fn $name(&self) -> &$type {
            &self.$name
        }
    };
}

impl<'a> Commit<'a> {
    getter!(hash, &'a str);
    getter!(date, &'a str);
    getter!(refs, Vec<&'a str>);
    getter!(subject, &'a str);

    fn build(
        hash: &'a str,
        date: &'a str,
        refs: Vec<&'a str>,
        subject: &'a str,
    ) -> anyhow::Result<Commit<'a>> {
        if hash.trim().is_empty() {
            return Err(anyhow!("missing hash"));
        }

        if date.trim().is_empty() {
            return Err(anyhow!("missing date"));
        }

        if subject.trim().is_empty() {
            return Err(anyhow!("missing subject"));
        }

        Ok(Commit {
            hash,
            date,
            refs,
            subject,
        })
    }
}

pub fn fetch_log() -> anyhow::Result<String> {
    let output = Command::new("git")
        .arg("log")
        .arg("--pretty=format:%H%x1f%cI%x1f%D%x1f%s%x1e")
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    Ok(stdout.to_string())
}

pub fn fetch_log_with_limit(limit: usize) -> anyhow::Result<String> {
    let output = Command::new("git")
        .arg("log")
        .arg("--pretty=format:%H%x1f%cI%x1f%D%x1f%s%x1e")
        .arg("-n")
        .arg(limit.to_string())
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    Ok(stdout.to_string())
}

/// commits must follow the format git log --pretty=format:%H%x1f%cI%x1f%D%x1f%s%x1e
pub fn parse_commit_log<'a>(format_string: &'a str) -> Vec<Commit<'a>> {
    let mut commits = Vec::new();

    for record in format_string.trim().split("\x1e") {
        if record.is_empty() {
            continue;
        }

        let mut fields = record.split("\x1f");

        let Some(hash) = fields.next() else {
            continue;
        };
        let hash = hash.trim();

        let Some(date) = fields.next() else {
            continue;
        };
        let date = date.trim();

        let Some(refs) = fields.next() else {
            continue;
        };
        let refs = refs
            .split(", ")
            .filter_map(|i| {
                let i = i.trim();

                if i.is_empty() { None } else { Some(i) }
            })
            .collect();

        let Some(subject) = fields.next() else {
            continue;
        };
        let subject = subject.trim();

        let Ok(commit) = Commit::build(hash, date, refs, subject) else {
            continue;
        };

        commits.push(commit);
    }

    commits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_parser_example() {
        let example = "1111111111111111111111111111111111111111\x1f2026-06-29T10:15:30+08:00\x1f\x1fInitial commit\x1e";

        let expected = Commit {
            hash: "1111111111111111111111111111111111111111",
            date: "2026-06-29T10:15:30+08:00",
            refs: vec![],
            subject: "Initial commit",
        };

        assert_eq!(parse_commit_log(example), vec![expected]);
    }

    #[test]
    fn test_log_parser_head_and_remote() {
        let example = "2222222222222222222222222222222222222222\x1f2026-06-29T11:20:00+08:00\x1fHEAD -> master, origin/master, origin/HEAD\x1fFix parser\x1e";

        let expected = Commit {
            hash: "2222222222222222222222222222222222222222",
            date: "2026-06-29T11:20:00+08:00",
            refs: vec!["HEAD -> master", "origin/master", "origin/HEAD"],
            subject: "Fix parser",
        };

        assert_eq!(parse_commit_log(example), vec![expected]);
    }

    #[test]
    fn test_log_parser_tag() {
        let example = "3333333333333333333333333333333333333333\x1f2026-06-28T18:45:10+08:00\x1ftag: v1.0.0\x1fRelease v1.0.0\x1e";

        let expected = Commit {
            hash: "3333333333333333333333333333333333333333",
            date: "2026-06-28T18:45:10+08:00",
            refs: vec!["tag: v1.0.0"],
            subject: "Release v1.0.0",
        };

        assert_eq!(parse_commit_log(example), vec![expected]);
    }

    #[test]
    fn test_log_parser_multiple_commits() {
        let example = concat!(
            "4444444444444444444444444444444444444444\x1f2026-06-27T09:00:00+08:00\x1ffeature/log-view\x1fAdd custom log view\x1e",
            "5555555555555555555555555555555555555555\x1f2026-06-26T14:30:25+08:00\x1forigin/feature/log-view\x1fAdd tests\x1e"
        );

        let expected = vec![
            Commit {
                hash: "4444444444444444444444444444444444444444",
                date: "2026-06-27T09:00:00+08:00",
                refs: vec!["feature/log-view"],
                subject: "Add custom log view",
            },
            Commit {
                hash: "5555555555555555555555555555555555555555",
                date: "2026-06-26T14:30:25+08:00",
                refs: vec!["origin/feature/log-view"],
                subject: "Add tests",
            },
        ];

        assert_eq!(parse_commit_log(example), expected);
    }

    #[test]
    fn test_log_parser_complex_subject() {
        let example = "6666666666666666666666666666666666666666\x1f2026-06-25T22:05:44+08:00\x1fHEAD -> main, origin/main\x1fFix: parse refs, dates, and subjects correctly\x1e";

        let expected = Commit {
            hash: "6666666666666666666666666666666666666666",
            date: "2026-06-25T22:05:44+08:00",
            refs: vec!["HEAD -> main", "origin/main"],
            subject: "Fix: parse refs, dates, and subjects correctly",
        };

        assert_eq!(parse_commit_log(example), vec![expected]);
    }

    #[test]
    fn test_log_parser_error_missing_hash() {
        let example = "\x1f2026-06-29T10:15:30+08:00\x1f\x1fInitial commit\x1e";

        assert!(parse_commit_log(example).is_empty());
    }

    #[test]
    fn test_log_parser_error_missing_date() {
        let example = "1111111111111111111111111111111111111111\x1f\x1f\x1fInitial commit\x1e";

        assert!(parse_commit_log(example).is_empty());
    }

    #[test]
    fn test_log_parser_error_missing_refs_field() {
        let example = "1111111111111111111111111111111111111111\x1f2026-06-29T10:15:30+08:00\x1fInitial commit\x1e";

        assert!(parse_commit_log(example).is_empty());
    }

    #[test]
    fn test_log_parser_error_missing_subject() {
        let example = "1111111111111111111111111111111111111111\x1f2026-06-29T10:15:30+08:00\x1fHEAD -> main\x1e";

        assert!(parse_commit_log(example).is_empty());
    }

    #[test]
    fn test_log_parser_garbage_input() {
        let exmaple = "123456NMHIYUGVJ{}[][vpdfkvfpovjfhue]]\\\x1e\x1e\x1e";

        assert!(parse_commit_log(exmaple).is_empty());
    }
}
