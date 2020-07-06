use log::{LevelFilter, SetLoggerError};
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};

pub fn init_logger() -> Result<(), SetLoggerError> {
    let file_path = "logs.log";
    let default_log_level = log::LevelFilter::Info;
    let log_format = "{h({d(%Y-%m-%d %H:%M:%S)})} {h({l})} {m}\n";

    // Build a stderr logger.
    let stderr = ConsoleAppender::builder().encoder(Box::new(PatternEncoder::new(log_format))).target(Target::Stderr).build();

    // Logging to log file.
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(log_format)))
        .build(file_path)
        .unwrap();

    // Set log level Debug for file logs
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(default_log_level)))
                .build("stderr", Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(LevelFilter::Debug),
        )
        .unwrap();

    // Use this to change log levels at runtime.
    // This means you can change the default log level to trace
    // if you are trying to debug an issue and need more logs on then turn it off
    // once you are done.
    let _handle = log4rs::init_config(config)?;

    Ok(())
}