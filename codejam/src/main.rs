#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(warnings)]
//#![feature(slice_patterns)]
//#![feature(range_contains)]
//use self::y2017qual::d::solve_all_cases;
//use self::y2017round1B::c::solve_all_cases;
//use self::y2017round1C::a::solve_all_cases;
//use self::y2017round1C::c::solve_all_cases;
use self::y2017round3::d::solve_all_cases;
use self::y2017round3::d::test_round3_d::*;

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
extern crate log4rs;
extern crate rand;

//mod algo_ebtech;

use self::util::log::init_log;

fn main()
{
    init_log();

    // test_2inf_grid_sum();
    solve_all_cases();
    //test_grid_sum();
}
