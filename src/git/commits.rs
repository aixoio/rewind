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
