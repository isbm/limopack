use log::{Record, Level, Metadata};

pub(crate) struct STDOUTLogger;

impl log::Log for STDOUTLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, msg: &Record) {
        if self.enabled(msg.metadata()) {
            println!("{}: {}", msg.level(), msg.args());
        }
    }

    fn flush(&self) {}
}
