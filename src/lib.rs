#![forbid(unsafe_code)]
#![deny(clippy::redundant_clone)]
#![deny(clippy::redundant_allocation)]
#![cfg_attr(not(test), deny(clippy::unwrap_used))]
#![cfg_attr(not(test), deny(clippy::expect_used))]

pub mod cli;
pub mod git;
