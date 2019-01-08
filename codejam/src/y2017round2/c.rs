/*
Round 3 2008

Round Qual 2012, hall of mirrors
*/
use super::super::util::input::*;
use super::super::util::math::*;
use std::cmp::max;

pub fn solve_all_cases()
{
    let mut reader = InputReader::new();
    let t = reader.read_int();

    for case in 1..=t {
        let (R, C) = reader.read_tuple_2::<u8, u8>();
        //P B
        for _ in 0..R {
            reader.read_chars(C as usize);
        }

        print!("{}", solve(case));
    }
}

fn solve(case_no: u32, ) -> String
{
    debug!("Solving case {}", case_no);

    format!("Case #{}: \n", case_no, )
}
