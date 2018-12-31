
//use y2017qual::a;
mod y2017qual;

#[macro_use] extern crate log;
extern crate log4rs;


use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};

use std::io::stdin;

fn main() {
    //Init logging
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("log/output.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
                   .appender("logfile")
                   .build(LevelFilter::Debug))
        .unwrap();

    let handler = log4rs::init_config(config).unwrap();

    //println!("Hello, world!");
    debug!("[bar] debug");
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let t = s.trim().parse::<u32>().unwrap();
    for case in 1..=t
    {
        debug!("Solving case {}", case);
        println!("Case #{}:", case);
        y2017qual::a::solve_case();        
    }
    
}
