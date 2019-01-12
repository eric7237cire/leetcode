#![allow(dead_code)]
#![allow(non_snake_case)]
//use self::y2017qual::d::solve_all_cases;
//use self::y2017round1B::c::solve_all_cases;
//use self::y2017round1C::a::solve_all_cases;
//use self::y2017round1C::c::solve_all_cases;
use self::y2017round2::d::solve_all_cases;

mod util;

mod algo;
mod y2017qual;
mod y2017round1A;
mod y2017round1B;
mod y2017round1C;
mod y2017round2;

#[macro_use]
extern crate log;
extern crate log4rs;

mod algo_ebtech;

use self::util::log::init_log;

fn main()
{
    //Init logging
    if false
    // cfg!(feature = "debug_print")
    {
        init_log();
    }

    solve_all_cases();
}
