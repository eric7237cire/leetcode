use crate::algo::graph::flow2::Flow;
use crate::util::codejam::run_cases;
use bit_set::BitSet;
use bit_vec::BitVec;
use rand::{thread_rng, Rng};
use std::cmp::max;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::Write;
use std::mem;
use std::usize;

use std::thread;

/*
Greedy alogrithm
*/
pub fn solve_all_cases()
{
    run_cases(
        &["C-small-practice", 
        //"B-large-practice"
        ],
        "y2016qual",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let (N, J) = reader.read_tuple_2();

                if case_no != 3 {
                    // continue;
                }

                println!("Solving case {}", case_no);

                writeln!(buffer, "Case #{}: {}", case_no, solve(N)).unwrap();
            }
        },
    );
}

fn solve(N: u16, J: u16) -> usize
{
    //brute force
    for jamcoin in 0..(1 << N) - 1 {
        println!("{:b}  N={}", jamcoin, N);
    } 
    
}
