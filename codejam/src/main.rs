//use self::y2017qual::a::solve_case;
#![allow(dead_code)]
//use self::y2017qual::d::solve_all_cases;
use self::y2017round1B::c::solve_all_cases;
//mod y2017qual;
//use self::util::input;
mod util;
#[allow(non_snake_case)]
mod y2017round1A;
#[allow(non_snake_case)]
mod y2017round1B;

#[macro_use]
extern crate log;
extern crate log4rs;

use self::util::log::init_log;

fn main()
{
    //Init logging
    if true
    // cfg!(feature = "debug_print")
    {
        init_log();
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
