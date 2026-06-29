use crate::git::log::fetch_log;

pub fn run() {
    let log = fetch_log().unwrap();
    println!("log..");
    println!("{log:#?}");
}
