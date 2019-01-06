use super::super::util::input::*;
//use std::u16;
use std::f64;
//use std::cmp::max;

pub fn solve_all_cases()
{
    let mut reader = InputReader::new();
    let t = reader.read_int();

    for case in 1..=t {
        let (_, P) = reader.read_tuple_2::<u8, u8>();
        let mut G: Vec<_> = reader.read_int_line::<u8>();

        print!("{}", solve(case));
    }
}

fn solve(case_no: u32) -> String
{
    debug!("Solving case {}", case_no);


    format!("Case #{}: \n", case_no)
}
