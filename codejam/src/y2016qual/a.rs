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
Simulation
Digits
*/
pub fn solve_all_cases()
{
    run_cases(
        &["A-small-practice", "A-large-practice"],
        "y2016qual",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let N = reader.read_int();

                if case_no != 3 {
                    // continue;
                }

                println!("Solving case {}", case_no);

                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case_no,
                    if let Some(ans) = solve(N) {
                        ans.to_string()
                    } else {
                        "INSOMNIA".to_string()
                    }
                )
                .unwrap();
            }
        },
    );
}

fn solve(N: u64) -> Option<u64>
{
    let mut digits_seen: BitSet = BitSet::new();

    for i in 1..1000 {
        let N = N * i;
        for c in N.to_string().chars() {
            let digit = c.to_digit(10).unwrap() as usize;
            digits_seen.insert(digit);
        }
        if digits_seen.len() == 10 {
            return Some(N);
        }
    }
    None
}
