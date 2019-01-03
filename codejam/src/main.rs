//use self::y2017qual::a::solve_case;
#![allow(dead_code)]
//use self::y2017qual::d::solve_all_cases;
use self::y2017round1a::c::solve_all_cases;
//mod y2017qual;
//use self::util::input;
mod util;
mod y2017round1a;

#[macro_use]
extern crate log;
extern crate log4rs;

use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

fn main()
{
    //Init logging
    if true
    // cfg!(feature = "debug_print")
    {
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

        let _handler = log4rs::init_config(config).unwrap();
    }

    solve_all_cases();
    /*
    Used in A,B,C
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let t = s.trim().parse::<u32>().unwrap();
    for case in 1..=t {
        debug!("Solving case {}", case);
        print!("Case #{}: ", case);
        solve_case();
    }
    */
}
