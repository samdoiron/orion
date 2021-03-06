// Logging utilities
// Copyright (C) 2015  Samuel Doiron, see LICENSE for details
use time;
use std::process::exit;

fn formatted_time() -> String {
    let now = time::now();
    format!("{}:{}:{}.{}", now.tm_hour, now.tm_min, now.tm_sec, now.tm_nsec / 1_000_000)
}

fn log_at_level(level: &'static str, message: &str) {
    println!("{} [{}] {}", formatted_time(), level, message);
}

pub fn info(message: &str) {
    log_at_level("INFO", message)
}

pub fn warn(message: &str) {
    log_at_level("WARN", message)
}

pub fn debug(message: &str) {
    log_at_level("DEBUG", message)
}

// TODO Log errors to stderr instead of stdout
pub fn error(message: &str) {
    log_at_level("ERROR", message)
}

// TODO Log errors to stderr instead of stdout
pub fn fatal(message: &str, code: i32) {
    log_at_level("FATAL", message);
    exit(code);
}

