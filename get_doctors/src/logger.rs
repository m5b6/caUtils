use colored::*;
use log::{Level, Metadata, Record, SetLoggerError};

pub struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level = match record.level() {
                Level::Error => "ERROR".red().bold(),
                Level::Warn => "WARN".yellow().bold(),
                Level::Info => "INFO".green().bold(),
                Level::Debug => "DEBUG".blue().bold(),
                Level::Trace => "TRACE".white().bold(),
            };
            println!("{} - {}", level, record.args());
        }
    }

    fn flush(&self) {}
}

pub fn init() -> Result<(), SetLoggerError> {
    log::set_boxed_logger(Box::new(SimpleLogger))
        .map(|()| log::set_max_level(log::LevelFilter::Info))
}
