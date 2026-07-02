#![cfg_attr(not(test), deny(clippy::unwrap_used))]
#![cfg_attr(not(test), deny(clippy::expect_used))]

pub mod branch;
pub mod log;
pub mod remote;
pub mod repo;
pub mod status;
pub mod tag;
