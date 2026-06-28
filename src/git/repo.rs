use std::path::Path;

pub fn is_git_repo() -> bool {
    Path::new(".git").exists()
}
