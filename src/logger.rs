use colored::{self, Colorize};
use log::{Level, Metadata, Record};

pub(crate) struct STDOUTLogger;

impl log::Log for STDOUTLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, msg: &Record) {
        if self.enabled(msg.metadata()) {
            let s_level: String;
            match msg.level() {
                log::Level::Info => s_level = format!("{}", msg.level().as_str().bright_green()),
                log::Level::Warn => s_level = format!("{}", msg.level().as_str().yellow()),
                log::Level::Error => s_level = format!("{}", msg.level().as_str().bright_red()),
                log::Level::Debug => s_level = format!("{}", msg.level().as_str().cyan()),
                log::Level::Trace => s_level = format!("{}", msg.level().as_str().cyan()),
            }

            println!("{}: {}", s_level, msg.args());
        }
    }

    fn flush(&self) {}
}
