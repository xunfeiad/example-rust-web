use log::debug;
use std::env;

// author: xunfei
// Project: untitled4
// File: log.rs
// Date: 2023/12/25 00:32
pub fn set_env() {
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    debug!("Starting Server...")
}
