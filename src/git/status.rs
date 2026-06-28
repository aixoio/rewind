use std::process::Command;

use anyhow::Ok;

#[derive(Debug)]
pub struct StatusResult {
    staged: Vec<String>,
    unstaged: Vec<String>,
    untracked: Vec<String>,
}

impl StatusResult {
    pub fn staged(&self) -> &Vec<String> {
        &self.staged
    }

    pub fn unstaged(&self) -> &Vec<String> {
        &self.unstaged
    }

    pub fn untracked(&self) -> &Vec<String> {
        &self.untracked
    }

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

    let mut result = StatusResult {
        staged: vec![],
        unstaged: vec![],
        untracked: vec![],
    };

    for line in stdout.lines() {
        let file_name: String = line.chars().skip(3).collect();

        if line.starts_with("??") {
            result.untracked.push(file_name);
            continue;
        }

        if !line.starts_with(" ") && !line.starts_with("?") {
            result.staged.push(file_name);
            continue;
        }

        let staged_char: String = line.chars().skip(1).collect();

        if (staged_char.starts_with("M") || staged_char.starts_with("D"))
            && !result.staged.contains(&file_name)
        {
            result.unstaged.push(file_name);
            continue;
        }
    }

    Ok(result)
}

pub fn fetch_status_raw() -> anyhow::Result<String> {
    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);

    Ok(stdout.to_string())
}
