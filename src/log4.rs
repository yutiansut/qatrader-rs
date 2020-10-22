extern crate log;
extern crate log4rs;

use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Logger, Root};

pub fn init_log4(path: &str, level: &str) {
    let lev = match level.to_uppercase().as_str() {
        "INFO" => LevelFilter::Info,
        "DEBUG" => LevelFilter::Debug,
        "ERROR" => LevelFilter::Error,
        "WARN" => LevelFilter::Warn,
        _ => LevelFilter::Info,
    };

    let stdout = ConsoleAppender::builder().build();

    let requests = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("[{d(%Y-%m-%d %H:%M:%S)}] [{l}] [thread:{I}] [{f}] [{t}]-  {m}{n}")))
        .build(path)
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("requests", Box::new(requests)))
        .logger(Logger::builder().build("app::backend::db", LevelFilter::Info))
        .logger(Logger::builder()
            .appender("requests")
            .additive(false)
            .build("app::requests", LevelFilter::Info))
        .build(Root::builder().appender("stdout").appender("requests").build(lev))
        .unwrap();

    let handle = log4rs::init_config(config).unwrap();
}