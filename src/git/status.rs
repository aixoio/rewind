use std::process::Command;

use anyhow::Ok;

#[derive(Debug)]
pub struct StatusResult {
    staged: Vec<String>,
    unstaged: Vec<String>,
    untracked: Vec<String>,
}

macro_rules! getter {
    ($name:ident, $type:ty) => {
        pub fn $name(&self) -> &$type {
            &self.$name
        }
    };
}

impl StatusResult {
    getter!(staged, Vec<String>);
    getter!(unstaged, Vec<String>);
    getter!(untracked, Vec<String>);

    pub fn total_files(&self) -> usize {
        self.staged.len() + self.unstaged.len() + self.untracked.len()
    }
}

pub fn fetch_status() -> anyhow::Result<StatusResult> {
    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);

    Ok(parse_status(&stdout))
}

pub fn fetch_status_raw() -> anyhow::Result<String> {
    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);

    Ok(stdout.to_string())
}

fn parse_status(status: &str) -> StatusResult {
    StatusResult {
        staged: vec![],
        unstaged: vec![],
        untracked: vec![],
    }
}
