#![allow(dead_code)]
#![allow(non_snake_case)]
//#![warn(clippy::all)]
//#![allow(warnings)]
//use self::y2017qual::d::solve_all_cases;
//use self::y2017round1B::c::solve_all_cases;
//use self::y2017round1C::a::solve_all_cases;
//use self::y2017round1C::c::solve_all_cases;
//use self::y2017round3;
//use self::y2017round3::d::solve_all_cases;
//use self::y2017round3::d::test_round3_d::*;

mod util;

mod algo;
/*mod y2017qual;
mod y2017round1A;
mod y2017round1B;
mod y2017round1C;
mod y2017round2;*/
mod y2017round3;

#[macro_use]
extern crate log;
#[macro_use]
extern crate try_opt;
//extern crate log4rs;
extern crate rand;

//mod algo_ebtech;

use self::util::log::init_log;

fn main()
{
    init_log();

    y2017round3::a::solve_all_cases();
    y2017round3::b::solve_all_cases();
    y2017round3::c::solve_all_cases();
    y2017round3::d::solve_all_cases();
}
