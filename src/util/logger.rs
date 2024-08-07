use std::fmt::{Display, Formatter};
use std::sync::Mutex;

use console::style;
use lazy_static::lazy_static;

#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub(crate) enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
    None,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Debug => {
                write!(f, "debug")
            }
            LogLevel::Info => {
                write!(f, "info")
            }
            LogLevel::Warn => {
                write!(f, "warn")
            }
            LogLevel::Error => {
                write!(f, "error")
            }
            LogLevel::None => {
                write!(f, "none")
            }
        }
    }
}

pub(crate) struct HexoLogger {
    level: LogLevel,
}

impl HexoLogger {
    pub(crate) fn level(&self) -> &LogLevel {
        &self.level
    }

    pub(crate) fn set_level(level: &LogLevel) {
        INSTANCE.lock().unwrap().level = level.clone();
    }

    pub(crate) fn error(&self, location: &str, message: &str) {
        eprintln!("{} :> {} {}", style("error").bright(), style(location).blue(), style(message).red());
    }

    pub(crate) fn info(&self, location: &str, message: &str) {
        eprintln!("{} :> {} {}", style("info").bright(), style(location).blue(), style(message).blue());
    }

    pub(crate) fn debug(&self, location: &str, message: &str) {
        println!("{} :> {} {}", style("debug").bright(),style(location).blue(), style(message));
    }

    pub(crate) fn warn(&self, location: &str, message: &str) {
        println!("{} :> {} {}", style("warn").bright(),style(location).blue(), style(message).yellow());
    }

    pub(crate) fn output(&self, message: &str) {
        println!("{message}");
    }
}

lazy_static!(
    pub(crate) static ref INSTANCE: Mutex<HexoLogger> =
        Mutex::new(
            HexoLogger {
                level: LogLevel::Debug
            }
    );
);

macro_rules! debug {
    ($($arg:tt)*) => {
        {
            let instance = crate::util::logger::INSTANCE.lock().unwrap();
            if *instance.level() <= crate::util::logger::LogLevel::Debug {
                instance.debug(
                    module_path!(),
                    format!($($arg)*).as_str()
                );
            }
        }
    };
}

macro_rules! error {
    ($($arg:tt)*) => {
        {
            let instance = crate::util::logger::INSTANCE.lock().unwrap();
            if *instance.level() <= crate::util::logger::LogLevel::Error {
                instance.error(
                    module_path!(),
                    format!($($arg)*).as_str()
                );
            }
        }
    };
}

macro_rules! info {
    ($($arg:tt)*) => {
        {
            let instance = crate::util::logger::INSTANCE.lock().unwrap();
            if *instance.level() <= crate::util::logger::LogLevel::Info {
                instance.info(
                    module_path!(),
                    format!($($arg)*).as_str()
                );
            }
        }
    };
}

macro_rules! warning {
    ($($arg:tt)*) => {
        {
            let instance = crate::util::logger::INSTANCE.lock().unwrap();
            if *instance.level() <= crate::util::logger::LogLevel::Warn {
                instance.warn(
                    module_path!(),
                    format!($($arg)*).as_str()
                );
            }
        }
    };
}

macro_rules! output {
    ($($arg:tt)*) => {
        let instance = crate::util::logger::INSTANCE.lock().unwrap();
        instance.output(
            format!($($arg)*).as_str()
        );
    };
}

pub(crate) use {debug, output, error, info, warning};