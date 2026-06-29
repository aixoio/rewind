use std::process::Command;

pub fn upstream() -> Option<String> {
    let Ok(output) = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("--symbolic-full-name")
        .arg("@{u}")
        .output()
    else {
        return None;
    };

    let stdout = String::from_utf8_lossy(&output.stdout);

    if !output.status.success() {
        return None;
    }

    Some(stdout.to_string())
}
