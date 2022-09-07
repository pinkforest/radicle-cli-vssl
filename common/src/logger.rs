//! Logging module.
use std::io;

pub use log::{Level, Log, Metadata, Record, SetLoggerError};

struct Logger {
    level: Level,
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let module = record.module_path().unwrap_or_default();

            if record.level() == Level::Error {
                write(record, module, io::stderr());
            } else {
                write(record, module, io::stdout());
            }

            fn write(record: &log::Record, module: &str, mut stream: impl io::Write) {
                writeln!(stream, "{} {} {}", record.level(), module, record.args()).ok();
            }
        }
    }

    fn flush(&self) {}
}

/// Initialize a new logger.
pub fn init(level: Level) -> Result<(), SetLoggerError> {
    let logger = Logger { level };

    log::set_boxed_logger(Box::new(logger))?;
    log::set_max_level(level.to_level_filter());

    Ok(())
}
