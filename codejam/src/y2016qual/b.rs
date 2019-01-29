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
        &["B-small-practice", "B-large-practice"],
        "y2016qual",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let N = reader.read_string().chars().map(|c| c == '+').collect();

                if case_no != 3 {
                    // continue;
                }

                println!("Solving case {}", case_no);

                writeln!(buffer, "Case #{}: {}", case_no, solve(N)).unwrap();
            }
        },
    );
}

fn solve(pancakes: BitVec) -> usize
{
    let mut flips = 0;
    //go through from bottom(right) to top, flipping as we needed
    for p in pancakes.iter().rev() {
        //Need a flip?  Take into account how many flips we have already done
        if (p && flips % 2 == 0) || (!p && flips % 2 == 1) {
            continue;
        }

        flips += 1;
    }

    flips
}
