use crate::git::status::fetch_status;

pub fn run() {
    let status = fetch_status().expect("failed to fetch git status");

    println!("{status:#?}");
}
