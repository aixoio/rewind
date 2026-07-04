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
