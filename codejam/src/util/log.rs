//use log::LevelFilter;

/*
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
*/

static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;

use log::{Level, LevelFilter, Metadata, Record};
struct ConsoleLogger;

impl log::Log for ConsoleLogger
{
    fn enabled(&self, metadata: &Metadata) -> bool
    {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record)
    {
        if self.enabled(record.metadata()) {
            println!("{}", record.args());
        }
    }

    fn flush(&self)
    {
    }
}

pub fn init_log()
{
    log::set_logger(&CONSOLE_LOGGER).unwrap();
    log::set_max_level(LevelFilter::Debug);

    //simple_logging::log_to_file("log/output.log", ::log::LevelFilter::Debug).unwrap();
    /*
    let logfile = FileAppender::builder()
        .append(false)
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("log/output.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("logfile")
                .build(LevelFilter::Debug),
        )
        .unwrap();

    let _handler = log4rs::init_config(config).unwrap();*/
}
