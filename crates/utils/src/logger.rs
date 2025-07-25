use std::fmt::Arguments;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Error = 1,
    Warn = 2,
    Info = 3,
    Debug = 4,
    Trace = 5,
}

impl LogLevel {
    fn as_str(self) -> &'static str {
        match self {
            LogLevel::Error => "ERROR",
            LogLevel::Warn => "WARN ",
            LogLevel::Info => "INFO ",
            LogLevel::Debug => "DEBUG",
            LogLevel::Trace => "TRACE",
        }
    }

    fn as_color(self) -> &'static str {
        match self {
            LogLevel::Error => "\x1b[91m",
            LogLevel::Warn => "\x1b[93m",
            LogLevel::Info => "\x1b[92m",
            LogLevel::Debug => "\x1b[94m",
            LogLevel::Trace => "\x1b[90m",
        }
    }

    fn icon(self) -> &'static str {
        match self {
            LogLevel::Error => "❌",
            LogLevel::Warn => "⚠️",
            LogLevel::Info => "ℹ️",
            LogLevel::Debug => "🐞",
            LogLevel::Trace => "🔍",
        }
    }
}

#[derive(Clone)]
pub struct Logger {
    level: LogLevel,
    silent: bool,
    stream: Arc<Mutex<dyn Write + Send>>,
}

impl Logger {
    pub fn new() -> Self {
        Self { level: LogLevel::Info, silent: false, stream: Arc::new(Mutex::new(io::stdout())) }
    }

    pub fn with_level(mut self, level: LogLevel) -> Self {
        self.level = level;
        self
    }

    pub fn silent(mut self, on: bool) -> Self {
        self.silent = on;
        self
    }

    pub fn with_stream<W: Write + Send + 'static>(mut self, writer: W) -> Self {
        self.stream = Arc::new(Mutex::new(writer));
        self
    }

    fn should_log(&self, level: LogLevel) -> bool {
        !self.silent && (level <= self.level)
    }

    fn log(&self, level: LogLevel, args: Arguments) {
        if !self.should_log(level) {
            return;
        }

        let mut out = self.stream.lock().unwrap();
        let icon = level.icon();
        let label = level.as_str();
        let color = level.as_color();
        let _ = write!(out, "{}{icon} {color}[{label}]\x1b[0m ", "");
        let _ = writeln!(out, "{}", args);
    }

    pub fn error(&self, message: Arguments) {
        self.log(LogLevel::Error, message);
    }

    pub fn warn(&self, message: Arguments) {
        self.log(LogLevel::Warn, message);
    }

    pub fn info(&self, message: Arguments) {
        self.log(LogLevel::Info, message);
    }

    pub fn debug(&self, message: Arguments) {
        self.log(LogLevel::Debug, message);
    }

    pub fn trace(&self, message: Arguments) {
        self.log(LogLevel::Trace, message);
    }
}

#[macro_export]
macro_rules! error {
    ($logger:expr, $($arg:tt)*) => {
        $logger.error(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! warn {
    ($logger:expr, $($arg:tt)*) => {
        $logger.warn(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! info {
    ($logger:expr, $($arg:tt)*) => {
        $logger.info(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! debug {
    ($logger:expr, $($arg:tt)*) => {
        $logger.debug(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! trace {
    ($logger:expr, $($arg:tt)*) => {
        $logger.trace(format_args!($($arg)*));
    };
}
