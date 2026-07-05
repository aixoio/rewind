use crate::getter;

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

/// only work with ouput from `git --no-pager stash list --pretty='format:%gd%x1f%cr%x1f%s%x1e'`
fn parse_stashes<'a>(input: &'a str) -> Vec<Stash<'a>> {
    let mut stashes = Vec::new();

    for record in input.split('\x1e') {
        if record.is_empty() {
            continue;
        }
        let mut data = record.split('\x1f');

        let Some(id) = data.next() else {
            continue;
        };
        let id = id.trim();

        let Some(created) = data.next() else {
            continue;
        };
        let created = created.trim();

        let Some(subject) = data.next() else {
            continue;
        };
        let subject = subject.trim();

        let stash = Stash {
            id,
            created,
            subject,
        };

        stashes.push(stash);
    }

    stashes
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

        assert_eq!(stashes.len(), 1);
        assert_eq!(stashes[0].id, "stash@{0}");
        assert_eq!(stashes[0].created, "5 minutes ago");
        assert_eq!(stashes[0].subject, "WIP on main: Test change");
    }

    #[test]
    fn returns_empty_vec_for_empty_input() {
        let stashes = parse_stashes("");

        assert!(stashes.is_empty());
    }

    #[test]
    fn ignores_empty_records() {
        let input = "\x1estash@{0}\x1f1 hour ago\x1fWIP on main: Change\x1e\x1e";

        let stashes = parse_stashes(input);

        assert_eq!(stashes.len(), 1);
        assert_eq!(stashes[0].id, "stash@{0}");
    }

    #[test]
    fn trims_whitespace_around_fields() {
        let input = "  stash@{0}  \x1f  2 weeks ago  \x1f  WIP on main: Change  \x1e";

        let stashes = parse_stashes(input);

        assert_eq!(stashes.len(), 1);
        assert_eq!(stashes[0].id, "stash@{0}");
        assert_eq!(stashes[0].created, "2 weeks ago");
        assert_eq!(stashes[0].subject, "WIP on main: Change");
    }

    #[test]
    fn skips_record_missing_created_field() {
        let input = concat!("stash@{0}\x1e", "stash@{1}\x1f1 day ago\x1fValid stash\x1e",);

        let stashes = parse_stashes(input);

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

        assert_eq!(stashes.len(), 1);
        assert_eq!(stashes[0].id, "stash@{1}");
    }

    #[test]
    fn preserves_colons_and_spaces_in_subject() {
        let input =
            "stash@{0}\x1f10 seconds ago\x1fWIP on feature/test: abc123 fix: handle spaces\x1e";

        let stashes = parse_stashes(input);

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

        assert_eq!(stashes.len(), 1);
        assert_eq!(stashes[0].id, "");
        assert_eq!(stashes[0].created, "");
        assert_eq!(stashes[0].subject, "");
    }

    #[test]
    fn ignores_extra_fields_after_subject() {
        let input = "stash@{0}\x1f1 hour ago\x1fValid subject\x1funexpected extra field\x1e";

        let stashes = parse_stashes(input);

        assert_eq!(stashes.len(), 1);
        assert_eq!(stashes[0].subject, "Valid subject");
    }

    #[test]
    fn random_data() {
        let input = "dvjndfjkvdfkj23546342qw";

        let stashes = parse_stashes(input);

        assert!(stashes.is_empty());
    }
}
