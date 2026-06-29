#[derive(Debug, PartialEq, Eq)]
pub struct Commit {
    hash: String,
    date: String,
    refs: Vec<String>,
    subject: String,
}

macro_rules! getter {
    ($name:ident, $type:ty) => {
        pub fn $name(&self) -> &$type {
            &self.$name
        }
    };
}

impl Commit {
    getter!(hash, String);
    getter!(date, String);
    getter!(refs, Vec<String>);
    getter!(subject, String);
}

/// commits must follow the format git log --pretty=format:%H%x1f%cI%x1f%D%x1f%s%x1e
fn parse_format_string(format_string: &str) -> anyhow::Result<Vec<Commit>> {
    Ok(vec![])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_parser_example() {
        let example = "1111111111111111111111111111111111111111\x1f2026-06-29T10:15:30+08:00\x1f\x1fInitial commit\x1e";

        let expected = Commit {
            hash: "1111111111111111111111111111111111111111".to_string(),
            date: "2026-06-29T10:15:30+08:00".to_string(),
            refs: vec![],
            subject: "Initial commit".to_string(),
        };

        assert_eq!(parse_format_string(example).unwrap(), vec![expected]);
    }

    #[test]
    fn test_log_parser_head_and_remote() {
        let example = "2222222222222222222222222222222222222222\x1f2026-06-29T11:20:00+08:00\x1fHEAD -> master, origin/master, origin/HEAD\x1fFix parser\x1e";

        let expected = Commit {
            hash: "2222222222222222222222222222222222222222".to_string(),
            date: "2026-06-29T11:20:00+08:00".to_string(),
            refs: vec![
                "HEAD -> master".to_string(),
                "origin/master".to_string(),
                "origin/HEAD".to_string(),
            ],
            subject: "Fix parser".to_string(),
        };

        assert_eq!(parse_format_string(example).unwrap(), vec![expected]);
    }

    #[test]
    fn test_log_parser_tag() {
        let example = "3333333333333333333333333333333333333333\x1f2026-06-28T18:45:10+08:00\x1ftag: v1.0.0\x1fRelease v1.0.0\x1e";

        let expected = Commit {
            hash: "3333333333333333333333333333333333333333".to_string(),
            date: "2026-06-28T18:45:10+08:00".to_string(),
            refs: vec!["tag: v1.0.0".to_string()],
            subject: "Release v1.0.0".to_string(),
        };

        assert_eq!(parse_format_string(example).unwrap(), vec![expected]);
    }

    #[test]
    fn test_log_parser_multiple_commits() {
        let example = concat!(
            "4444444444444444444444444444444444444444\x1f2026-06-27T09:00:00+08:00\x1ffeature/log-view\x1fAdd custom log view\x1e",
            "5555555555555555555555555555555555555555\x1f2026-06-26T14:30:25+08:00\x1forigin/feature/log-view\x1fAdd tests\x1e"
        );

        let expected = vec![
            Commit {
                hash: "4444444444444444444444444444444444444444".to_string(),
                date: "2026-06-27T09:00:00+08:00".to_string(),
                refs: vec!["feature/log-view".to_string()],
                subject: "Add custom log view".to_string(),
            },
            Commit {
                hash: "5555555555555555555555555555555555555555".to_string(),
                date: "2026-06-26T14:30:25+08:00".to_string(),
                refs: vec!["origin/feature/log-view".to_string()],
                subject: "Add tests".to_string(),
            },
        ];

        assert_eq!(parse_format_string(example).unwrap(), expected);
    }

    #[test]
    fn test_log_parser_complex_subject() {
        let example = "6666666666666666666666666666666666666666\x1f2026-06-25T22:05:44+08:00\x1fHEAD -> main, origin/main\x1fFix: parse refs, dates, and subjects correctly\x1e";

        let expected = Commit {
            hash: "6666666666666666666666666666666666666666".to_string(),
            date: "2026-06-25T22:05:44+08:00".to_string(),
            refs: vec!["HEAD -> main".to_string(), "origin/main".to_string()],
            subject: "Fix: parse refs, dates, and subjects correctly".to_string(),
        };

        assert_eq!(parse_format_string(example).unwrap(), vec![expected]);
    }

    #[test]
    fn test_log_parser_error_missing_hash() {
        let example = "\x1f2026-06-29T10:15:30+08:00\x1f\x1fInitial commit\x1e";

        assert!(parse_format_string(example).is_err());
    }

    #[test]
    fn test_log_parser_error_missing_date() {
        let example = "1111111111111111111111111111111111111111\x1f\x1f\x1fInitial commit\x1e";

        assert!(parse_format_string(example).is_err());
    }

    #[test]
    fn test_log_parser_error_missing_refs_field() {
        let example = "1111111111111111111111111111111111111111\x1f2026-06-29T10:15:30+08:00\x1fInitial commit\x1e";

        assert!(parse_format_string(example).is_err());
    }

    #[test]
    fn test_log_parser_error_missing_subject() {
        let example = "1111111111111111111111111111111111111111\x1f2026-06-29T10:15:30+08:00\x1fHEAD -> main\x1e";

        assert!(parse_format_string(example).is_err());
    }

    #[test]
    fn test_log_parser_error_too_many_fields() {
        let example = "1111111111111111111111111111111111111111\x1f2026-06-29T10:15:30+08:00\x1fHEAD -> main\x1fInitial commit\x1fextra\x1e";

        assert!(parse_format_string(example).is_err());
    }
}
