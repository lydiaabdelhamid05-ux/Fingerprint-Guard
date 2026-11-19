//! Independent, local-only file integrity monitoring library.
pub mod db;
pub mod security;
pub mod verify;

pub use verify::{check_file, enroll_file, CheckResult};
