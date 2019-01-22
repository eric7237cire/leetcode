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
use byteorder::{NativeEndian,WriteBytesExt,ByteOrder};
use hamming::weight;

/*
Dynamic programming, min/max
Arithmetic
*/
pub fn solve_all_cases()
{
    run_cases(
        &["B-small-practice", 
        //"BA-large-practice"],
        ],
        "y2017round4",
        |reader, buffer| {
            let t = reader.read_int();

            for case in 1..=t {
                let (S,C) = reader.read_tuple_2::<u16>();

                let cards = (0..C).map(|_| {
                    let mut sw = reader.read_string().split_whitespace();
                    (sw.next().unwrap().parse::<char>().unwrap(),
                     sw.next().unwrap().parse::<u16>().unwrap())
                }).collect();

                write!(buffer, "{}", solve(case, &cards, S)).unwrap();
            }
        },
    );
}

fn solve(case_no: u32, cards: &Vec<(char, u16)>, S: u16) -> String
{
    let mut bits = vec![ vec![0u16;0]; 16] ;

    for i in 0..1u16<<15 {
        let mut bytes : [u8; 2] = [0;2];
        NativeEndian::write_u16(&mut bytes, i);

        let pop_count = weight(&bytes);

        bits[pop_count as usize].push(i);
    }

    println!("Solving case {}", case_no);

    format!("Case #{}: \n", case_no)
}
