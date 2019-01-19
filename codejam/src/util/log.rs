//use log::LevelFilter;

/*
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
*/

pub fn init_log()
{
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
