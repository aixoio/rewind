#[derive(Debug, PartialEq)]
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
    fn test_log_parser_example_1() {
        let example = "1111111111111111111111111111111111111111\x1f2026-06-29T10:15:30+08:00\x1f\x1fInitial commit\x1e";
        let expected = Commit {
            hash: "1111111111111111111111111111111111111111".to_string(),
            date: "2026-06-29T10:15:30+08:00".to_string(),
            refs: vec![],
            subject: "Initial commit".to_string(),
        };

        let result = parse_format_string(example).unwrap();

        assert_eq!(result, vec![expected]);
    }
}
