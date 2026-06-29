#[derive(Debug)]
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
