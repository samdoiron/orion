fn log_at_level(level: &'static str, message: &str) {
    println!("[{}] {}", level, message);
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
pub fn fatal(message: &str) {
    log_at_level("FATAL", message)
}

