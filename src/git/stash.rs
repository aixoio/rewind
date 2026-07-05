use std::process::{Command, Stdio};

use anyhow::anyhow;

use crate::getter;

#[derive(Debug)]
pub struct Stash<'a> {
    id: &'a str,
    created: &'a str,
    subject: &'a str,
}

impl<'a> Stash<'a> {
    getter!(id, &'a str);
    getter!(created, &'a str);
    getter!(subject, &'a str);
}

pub fn fetch_stashes() -> anyhow::Result<String> {
    let output = Command::new("git")
        .arg("--no-pager")
        .arg("stash")
        .arg("list")
        .arg("--pretty=format:%gd%x1f%cr%x1f%s%x1e")
        .output()?;
    let stdout = String::from_utf8(output.stdout)?;

    if !output.status.success() {
        return Err(anyhow!(
            "error: git: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(stdout)
}

pub fn pop_stash() -> anyhow::Result<String> {
    let output = Command::new("git")
        .arg("stash")
        .arg("pop")
        .stdout(Stdio::inherit())
        .output()?;
    let stdout = String::from_utf8(output.stdout)?;

    if !output.status.success() {
        return Err(anyhow!(
            "error: git: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(stdout)
}

pub fn push_stash(message: &str) -> anyhow::Result<()> {
    let output = Command::new("git")
        .arg("stash")
        .arg("push")
        .arg("--include-untracked")
        .arg("-m")
        .arg(message)
        .stdout(Stdio::inherit())
        .output()?;

    if !output.status.success() {
        return Err(anyhow!(
            "error: git: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}

/// only work with ouput from `git --no-pager stash list --pretty='format:%gd%x1f%cr%x1f%s%x1e'`
pub fn parse_stashes<'a>(input: &'a str) -> impl Iterator<Item = Stash<'a>> + 'a {
    input.split('\x1e').filter_map(|record| {
        if record.is_empty() {
            return None;
        }
        let mut data = record.split('\x1f');

        let id = data.next()?.trim();
        let created = data.next()?.trim();
        let subject = data.next()?.trim();

        Some(Stash {
            id,
            created,
            subject,
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_multiple_stashes() {
        let input = concat!(
            "stash@{0}\x1f2 hours ago\x1fWIP on main: abc123 First change\x1e",
            "stash@{1}\x1f3 days ago\x1fOn feature: Add login\x1e",
        );

        let stashes = parse_stashes(input);
        let stashes: Vec<_> = stashes.collect();

        assert_eq!(stashes.len(), 2);

        assert_eq!(stashes[0].id, "stash@{0}");
        assert_eq!(stashes[0].created, "2 hours ago");
        assert_eq!(stashes[0].subject, "WIP on main: abc123 First change");

        assert_eq!(stashes[1].id, "stash@{1}");
        assert_eq!(stashes[1].created, "3 days ago");
        assert_eq!(stashes[1].subject, "On feature: Add login");
    }

    #[test]
    fn parses_output_without_trailing_record_separator() {
        let input = "stash@{0}\x1f5 minutes ago\x1fWIP on main: Test change";

        let stashes = parse_stashes(input);
        let stashes: Vec<_> = stashes.collect();

        assert_eq!(stashes.len(), 1);
        assert_eq!(stashes[0].id, "stash@{0}");
        assert_eq!(stashes[0].created, "5 minutes ago");
        assert_eq!(stashes[0].subject, "WIP on main: Test change");
    }

    #[test]
    fn returns_empty_vec_for_empty_input() {
        let stashes = parse_stashes("");
        let stashes: Vec<_> = stashes.collect();

        assert!(stashes.is_empty());
    }

    #[test]
    fn ignores_empty_records() {
        let input = "\x1estash@{0}\x1f1 hour ago\x1fWIP on main: Change\x1e\x1e";

        let stashes = parse_stashes(input);
        let stashes: Vec<_> = stashes.collect();

        assert_eq!(stashes.len(), 1);
        assert_eq!(stashes[0].id, "stash@{0}");
    }

    #[test]
    fn trims_whitespace_around_fields() {
        let input = "  stash@{0}  \x1f  2 weeks ago  \x1f  WIP on main: Change  \x1e";

        let stashes = parse_stashes(input);
        let stashes: Vec<_> = stashes.collect();

        assert_eq!(stashes.len(), 1);
        assert_eq!(stashes[0].id, "stash@{0}");
        assert_eq!(stashes[0].created, "2 weeks ago");
        assert_eq!(stashes[0].subject, "WIP on main: Change");
    }

    #[test]
    fn skips_record_missing_created_field() {
        let input = concat!("stash@{0}\x1e", "stash@{1}\x1f1 day ago\x1fValid stash\x1e",);

        let stashes = parse_stashes(input);
        let stashes: Vec<_> = stashes.collect();

        assert_eq!(stashes.len(), 1);
        assert_eq!(stashes[0].id, "stash@{1}");
    }

    #[test]
    fn skips_record_missing_subject_field() {
        let input = concat!(
            "stash@{0}\x1f1 day ago\x1e",
            "stash@{1}\x1f2 days ago\x1fValid stash\x1e",
        );

        let stashes = parse_stashes(input);
        let stashes: Vec<_> = stashes.collect();

        assert_eq!(stashes.len(), 1);
        assert_eq!(stashes[0].id, "stash@{1}");
    }

    #[test]
    fn preserves_colons_and_spaces_in_subject() {
        let input =
            "stash@{0}\x1f10 seconds ago\x1fWIP on feature/test: abc123 fix: handle spaces\x1e";

        let stashes = parse_stashes(input);
        let stashes: Vec<_> = stashes.collect();

        assert_eq!(stashes.len(), 1);
        assert_eq!(
            stashes[0].subject,
            "WIP on feature/test: abc123 fix: handle spaces"
        );
    }

    #[test]
    fn allows_empty_fields_produced_by_git_format() {
        let input = "\x1f\x1f\x1e";

        let stashes = parse_stashes(input);
        let stashes: Vec<_> = stashes.collect();

        assert_eq!(stashes.len(), 1);
        assert_eq!(stashes[0].id, "");
        assert_eq!(stashes[0].created, "");
        assert_eq!(stashes[0].subject, "");
    }

    #[test]
    fn ignores_extra_fields_after_subject() {
        let input = "stash@{0}\x1f1 hour ago\x1fValid subject\x1funexpected extra field\x1e";

        let stashes = parse_stashes(input);
        let stashes: Vec<_> = stashes.collect();

        assert_eq!(stashes.len(), 1);
        assert_eq!(stashes[0].subject, "Valid subject");
    }

    #[test]
    fn random_data() {
        let input = "dvjndfjkvdfkj23546342qw";

        let stashes = parse_stashes(input);
        let stashes: Vec<_> = stashes.collect();

        assert!(stashes.is_empty());
    }
}
