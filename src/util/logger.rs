use std::fmt::format;
use lazy_static::lazy_static;

#[derive(PartialOrd, PartialEq)]
pub(crate) enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
    None
}

pub(crate) struct HexoLogger {
    level: LogLevel
}

impl HexoLogger {

    pub(crate) fn level(&self) -> &LogLevel {
        return &self.level
    }

    pub(crate) fn debug(&self, message: &str) {
        println!("{message}");
    }
}

lazy_static!(
    pub(crate) static ref INSTANCE: HexoLogger = HexoLogger {
        level: LogLevel::Debug
    };
);

macro_rules! debug {
    ($($arg:tt)*) => {
        use crate::util::logger::LogLevel;
        if *crate::util::logger::INSTANCE.level() >= LogLevel::Debug {
            crate::util::logger::INSTANCE.debug(
                format!(
                    "{}: {}",
                    module_path!(),
                    format!($($arg)*).as_str()
                ).as_str()
            );
        }
    };

}

pub(crate) use debug;