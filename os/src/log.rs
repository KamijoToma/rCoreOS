use log::{Level, Metadata, Record, SetLoggerError};

use crate::println;

// Reference: https://docs.rs/log/0.4.14/log/#implementing-a-logger

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        let color = match record.level() {
            Level::Trace => 90,
            Level::Debug => 32,
            Level::Info => 34,
            Level::Warn => 93,
            Level::Error => 31,
        };
        if self.enabled(record.metadata()) {
            println!(
                "\u{1b}[{}m[{}] {}\u{1b}[0m",
                color,
                record.level(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(log::LevelFilter::Trace))
}
