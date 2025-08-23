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

fn log_line(
    symbol_colored: impl Display,
    msg_colored: impl Display,
    time: bool,
    stderr: bool,
    indent: usize,
) {
    let timestamp_str = if time { format!("{} ", timestamp().dimmed()) } else { String::new() };
    let indent_str = "  ".repeat(indent);
    let line = format!("{timestamp_str}{indent_str}{symbol_colored} {msg_colored}");

    if stderr {
        eprintln!("{line}");
    } else {
        println!("{line}");
    }
}

pub fn info(msg: impl AsRef<str>, time: bool) {
    log_line("ℹ".blue().bold(), msg.as_ref().bright_blue(), time, false, 0);
}

pub fn success(msg: impl AsRef<str>, time: bool) {
    log_line("✔".green().bold(), msg.as_ref().bright_green(), time, false, 0);
}

pub fn warn(msg: impl AsRef<str>, time: bool) {
    log_line("⚠".yellow().bold(), msg.as_ref().bright_yellow(), time, false, 0);
}

pub fn error(msg: impl AsRef<str>, time: bool) {
    log_line("✘".red().bold(), msg.as_ref().bright_red(), time, true, 0);
}

pub fn debug(msg: impl AsRef<str>, time: bool) {
    if DEBUG_ENABLED.load(Ordering::Relaxed) {
        log_line("🐞".magenta().bold(), msg.as_ref().bright_magenta(), time, false, 0);
    }
}

pub fn sub_info(msg: impl AsRef<str>, time: bool) {
    log_line("└".bright_black().bold(), msg.as_ref().bright_black(), time, false, 1);
}

pub fn sub_success(msg: impl AsRef<str>, time: bool) {
    log_line("└".bright_black().bold(), msg.as_ref().bright_black(), time, false, 1);
}

pub fn sub_warn(msg: impl AsRef<str>, time: bool) {
    log_line("└".bright_black().bold(), msg.as_ref().bright_black(), time, false, 1);
}

pub fn sub_error(msg: impl AsRef<str>, time: bool) {
    log_line("└".bright_black().bold(), msg.as_ref().bright_black(), time, true, 1);
}

pub fn sub_debug(msg: impl AsRef<str>, time: bool) {
    if DEBUG_ENABLED.load(Ordering::Relaxed) {
        log_line("└".bright_black().bold(), msg.as_ref().bright_black(), time, false, 1);
    }
}

pub fn sub_log(msg: impl AsRef<str>, time: bool, stderr: bool) {
    log_line("└".bright_black().bold(), msg.as_ref().bright_black(), time, stderr, 1);
}

pub fn sub_sub_log(msg: impl AsRef<str>, time: bool, stderr: bool) {
    log_line("  └".bright_black().bold(), msg.as_ref().bright_black(), time, stderr, 2);
}

pub fn step(step_num: usize, total_steps: usize, msg: impl AsRef<str>, time: bool) {
    let step = format!("[{step_num}/{total_steps}]");
    let step_cyan = step.cyan();
    let step_bold = step_cyan.bold();
    log_line(step_bold, msg.as_ref().bright_cyan(), time, false, 0);
}

pub fn sub_step(msg: impl AsRef<str>, time: bool) {
    log_line("├".bright_black().bold(), msg.as_ref().bright_black(), time, false, 1);
}

pub fn separator(title: impl AsRef<str>) {
    let separator_line = format!("─── {} ───", title.as_ref());
    let separator_bright = separator_line.bright_black();
    let separator_bold = separator_bright.bold();
    println!("{separator_bold}");
}
