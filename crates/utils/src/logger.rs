use once_cell::sync::Lazy;
use owo_colors::OwoColorize;
use std::fmt::Display;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static DEBUG_ENABLED: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));

pub fn enable_debug() {
    DEBUG_ENABLED.store(true, Ordering::Relaxed);
}

fn timestamp() -> String {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    format!("[{}]", now.as_secs())
}

fn log_line(symbol_colored: impl Display, msg_colored: impl Display, time: bool, stderr: bool) {
    let timestamp_str = if time { format!("{} ", timestamp().dimmed()) } else { String::new() };

    let line = format!("{}{} {}", timestamp_str, symbol_colored, msg_colored);

    if stderr {
        eprintln!("{}", line);
    } else {
        println!("{}", line);
    }
}

pub fn info(msg: impl AsRef<str>, time: bool) {
    log_line("ℹ".blue().bold(), msg.as_ref().bright_blue(), time, false);
}

pub fn success(msg: impl AsRef<str>, time: bool) {
    log_line("✔".green().bold(), msg.as_ref().bright_green(), time, false);
}

pub fn warn(msg: impl AsRef<str>, time: bool) {
    log_line("⚠".yellow().bold(), msg.as_ref().bright_yellow(), time, false);
}

pub fn error(msg: impl AsRef<str>, time: bool) {
    log_line("✘".red().bold(), msg.as_ref().bright_red(), time, true);
}

pub fn debug(msg: impl AsRef<str>, time: bool) {
    if DEBUG_ENABLED.load(Ordering::Relaxed) {
        log_line("🐞".magenta().bold(), msg.as_ref().bright_magenta(), time, false);
    }
}
