use crate::getter;

#[derive(Debug, PartialEq, Eq)]
pub struct CommitInfo {
    hash: String,
    message: String,
    author: String,
    date: String,
}

impl CommitInfo {
    getter!(hash, String);
    getter!(message, String);
    getter!(author, String);
    getter!(date, String);
}

/// input must come from `git show --no-patch --format=%H%x1f%s%x1f%an%x1f%ad --date=short`
pub fn parse_commit_info(input: &str) -> Option<CommitInfo> {
    let mut data = input.trim().split('\x1f');

    let hash = data.next()?.to_string();
    let message = data.next()?.to_string();
    let author = data.next()?.to_string();
    let date = data.next()?.to_string();

    Some(CommitInfo {
        hash,
        message,
        author,
        date,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_commit_info_parses_valid_git_show_output() {
        let input = "abc123\x1fInitial commit\x1fJane Doe\x1f2026-07-04";

        let info = parse_commit_info(input).unwrap();

        assert_eq!(info.hash(), &"abc123".to_string());
        assert_eq!(info.message(), &"Initial commit".to_string());
        assert_eq!(info.author(), &"Jane Doe".to_string());
        assert_eq!(info.date(), &"2026-07-04".to_string());
    }

    #[test]
    fn parse_commit_info_trims_surrounding_whitespace() {
        let input = "\nabc123\x1fFix bug\x1fJohn Smith\x1f2026-01-15\n";

        let info = parse_commit_info(input).unwrap();

        assert_eq!(
            info,
            CommitInfo {
                hash: "abc123".to_string(),
                message: "Fix bug".to_string(),
                author: "John Smith".to_string(),
                date: "2026-01-15".to_string(),
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
                hash: "abc123".to_string(),
                message: "Initial commit".to_string(),
                author: "Jane Doe".to_string(),
                date: "2026-07-04".to_string(),
            }
        );
    }

    #[test]
    fn parse_commit_info_allows_empty_fields() {
        let input = "\x1f\x1f\x1f";

        let info = parse_commit_info(input).unwrap();

        assert_eq!(
            info,
            CommitInfo {
                hash: "".to_string(),
                message: "".to_string(),
                author: "".to_string(),
                date: "".to_string(),
            }
        );
    }
}
