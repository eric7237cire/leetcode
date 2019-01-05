use super::super::util::input::*;
use itertools::Itertools;
//use std::fmt;
//use std::io::stdin;
//use std::iter::FromIterator;

pub fn solve_all_cases()
{
    //let mut children: Vec<thread::JoinHandle<_>> = vec![];
    let mut reader = InputReader::new();
    let t = reader.read_int();

    for case in 1..=t
    {
        //N, R, O(RY), Y, G(YB), B, and V(RB).
        let (N,Q) = reader.read_int_line_iter::<u8>().tuples().next().unwrap();

        //  children.push(thread::spawn(move || -> String { solve(case, &input) }));
        print!("{}", solve(case));
    }    /*
    for child in children
    {
        print!("{}", child.join().unwrap());
    }*/
}


#[allow(non_snake_case)]
fn solve(case_no: u32) -> String
{
    debug!("Solving case {}", case_no);

    format!(
        "Case #{}: \n",
        case_no,

    )
}
