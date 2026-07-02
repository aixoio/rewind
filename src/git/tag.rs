use std::process::Command;

use anyhow::anyhow;

#[derive(Debug, PartialEq, Eq)]
pub struct Tag<'a> {
    name: &'a str,
    relative_date: &'a str,
    subject: &'a str,
}

macro_rules! getter {
    ($name:ident, $type:ty) => {
        pub fn $name(&self) -> &$type {
            &self.$name
        }
    };
}

impl<'a> Tag<'a> {
    getter!(name, &'a str);
    getter!(relative_date, &'a str);
    getter!(subject, &'a str);

    fn build(name: &'a str, relative_date: &'a str, subject: &'a str) -> anyhow::Result<Tag<'a>> {
        if name.trim().is_empty() {
            return Err(anyhow!("missing name"));
        }

        if relative_date.trim().is_empty() {
            return Err(anyhow!("missing date"));
        }

        if subject.trim().is_empty() {
            return Err(anyhow!("missing subject"));
        }

        Ok(Tag {
            name,
            relative_date,
            subject,
        })
    }
}

pub fn fetch_all_tags() -> anyhow::Result<String> {
    let output = Command::new("git")
        .arg("--no-pager")
        .arg("tag")
        .arg("-l")
        .arg("--sort=-creatordate")
        .arg("--format=%(refname:short)%1f%(creatordate:relative)%1f%(subject)%1e")
        .output()?;

    if !output.status.success() {
        return Err(anyhow!(
            "error: git: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    Ok(stdout.to_string())
}

pub fn create_lightweight_tag(name: &str) -> anyhow::Result<()> {
    let output = Command::new("git").arg("tag").arg(name).output()?;

    if !output.status.success() {
        return Err(anyhow!(
            "error: git: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}

/// input must be from the `git --no-pager tag -l --sort=-creatordate --format='%(refname:short)%1f%(creatordate:relative)%1f%(subject)%1e'` command
/// or from fetch_all_tags()
pub fn parse_git_tags<'a>(format_string: &'a str) -> Vec<Tag<'a>> {
    let mut tags = Vec::new();

    for line in format_string.split("\x1e") {
        if line.trim().is_empty() {
            continue;
        }

        let mut formated_line = line.trim().split("\x1f");

        let Some(name) = formated_line.next() else {
            continue;
        };
        let name = name.trim();

        let Some(relative_date) = formated_line.next() else {
            continue;
        };
        let relative_date = relative_date.trim();

        let Some(subject) = formated_line.next() else {
            continue;
        };
        let subject = subject.trim();

        let Ok(tag) = Tag::build(name, relative_date, subject) else {
            continue;
        };

        tags.push(tag);
    }

    tags
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIELD_SEPARATOR: char = '\x1f';
    const RECORD_SEPARATOR: char = '\x1e';

    #[test]
    fn parses_a_single_tag() {
        let input = "v1.0.0\x1f2 weeks ago\x1fInitial release\x1e";

        let tags = parse_git_tags(input);

        assert_eq!(
            tags,
            vec![Tag {
                name: "v1.0.0",
                relative_date: "2 weeks ago",
                subject: "Initial release",
            }]
        );
    }

    #[test]
    fn parses_multiple_tags() {
        let input = concat!(
            "v2.0.0\x1f3 days ago\x1fSecond major release\x1e",
            "v1.1.0\x1f2 months ago\x1fAdd authentication\x1e",
            "v1.0.0\x1f1 year ago\x1fInitial release\x1e",
        );

        let tags = parse_git_tags(input);

        assert_eq!(
            tags,
            vec![
                Tag {
                    name: "v2.0.0",
                    relative_date: "3 days ago",
                    subject: "Second major release",
                },
                Tag {
                    name: "v1.1.0",
                    relative_date: "2 months ago",
                    subject: "Add authentication",
                },
                Tag {
                    name: "v1.0.0",
                    relative_date: "1 year ago",
                    subject: "Initial release",
                },
            ]
        );
    }

    #[test]
    fn returns_an_empty_vec_for_empty_input() {
        assert!(parse_git_tags("").is_empty());
    }

    #[test]
    fn skips_a_record_with_too_few_fields() {
        let input = "v1.0.0\x1fyesterday\x1e";

        assert!(parse_git_tags(input).is_empty());
    }

    #[test]
    fn skips_a_record_with_too_many_fields() {
        let input = "v1.0.0\x1fyesterday\x1fRelease\x1funexpected\x1e";

        let tags = parse_git_tags(input);

        assert_eq!(
            tags,
            vec![Tag {
                name: "v1.0.0",
                relative_date: "yesterday",
                subject: "Release",
            }]
        );
    }

    #[test]
    fn skips_malformed_records_but_keeps_valid_records() {
        let input = concat!(
            "malformed record\x1e",
            "v2.0.0\x1f4 days ago\x1fValid release\x1e",
            "v1.0.0\x1fmissing subject\x1e",
        );

        let tags = parse_git_tags(input);

        assert_eq!(
            tags,
            vec![Tag {
                name: "v2.0.0",
                relative_date: "4 days ago",
                subject: "Valid release",
            }]
        );
    }

    #[test]
    fn supports_empty_field_values() {
        let input = "\x1f\x1f\x1e";

        let tags = parse_git_tags(input);

        assert!(tags.is_empty());
    }

    #[test]
    fn does_not_treat_commas_spaces_or_pipes_as_separators() {
        let input =
            "release|v1, stable\x1fabout 1 month ago\x1fFix parsing, formatting | output\x1e";

        let tags = parse_git_tags(input);

        assert_eq!(
            tags,
            vec![Tag {
                name: "release|v1, stable",
                relative_date: "about 1 month ago",
                subject: "Fix parsing, formatting | output",
            }]
        );
    }

    #[test]
    fn separators_have_the_expected_values() {
        assert_eq!(FIELD_SEPARATOR, '\u{001f}');
        assert_eq!(RECORD_SEPARATOR, '\u{001e}');
    }
}
