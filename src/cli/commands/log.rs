use crate::git::log::fetch_log;

pub fn run(limit: Option<usize>) {
    let log = fetch_log().unwrap();
    println!("log..");
    println!("{log:#?}");
}
