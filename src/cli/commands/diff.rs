use crate::git::repo;

pub fn run() {
    repo::diff().expect("failed to diff");
}
