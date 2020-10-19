use log::{Level, Metadata, Record};
use polyhorn_android_sys::{android_log_write, AndroidLogPriority};

pub struct AndroidLogger;

impl log::Log for AndroidLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        android_log_write(
            match record.level() {
                Level::Error => AndroidLogPriority::Error,
                Level::Warn => AndroidLogPriority::Warn,
                Level::Info => AndroidLogPriority::Info,
                Level::Debug => AndroidLogPriority::Debug,
                Level::Trace => AndroidLogPriority::Verbose,
            },
            "polyhorn",
            &record.args().to_string(),
        )
    }

    fn flush(&self) {}
}
