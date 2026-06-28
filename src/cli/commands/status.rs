use crate::git::status::fetch_status;

pub fn run() {
    let status = fetch_status().unwrap();

    println!("{status:#?}");
}
