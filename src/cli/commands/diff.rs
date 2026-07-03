use crate::{
    check_for_git_repo,
    git::repo::{self, is_git_repo},
    handle_error,
};

pub fn run() {
    check_for_git_repo!();

    handle_error!(repo::diff());
}
