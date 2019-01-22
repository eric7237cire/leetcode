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
use num_bigint::BigInt;
use num_rational::{BigRational, Ratio};
use num_integer::Integer;
use num_traits::FromPrimitive;
use std::ops::{Add, Sub, Mul, Div};

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
                let (S,C) = reader.read_tuple_2::<i16>();

                let cards = (0..C).map(|_| {
                    let mut sw = reader.read_string().split_whitespace();
                    (sw.next().unwrap().parse::<char>().unwrap(),
                     sw.next().unwrap().parse::<i16>().unwrap())
                }).collect();

                write!(buffer, "{}", solve(case, &cards, S)).unwrap();
            }
        },
    );
}

#[derive(Clone)]
struct MemoData
{
    high: BigRational,
    low: BigRational,
}

fn apply_op(card: &(char, BigRational), num: &BigRational) -> BigRational

{
    let num = num.clone();
    if card.0 == '+' {
       num + &card.1
    } else if card.0 == '-' {
        num - &card.1
    } else if card.0 == '*' {
        num * &card.1
    } else if card.0 == '/' {
        num / &card.1
    } else {
        assert!(false);
        num * BigRational::from_i8(1).unwrap()
    }
}

fn solve(case_no: u32, cards: &Vec<(char, i16)>, S: i16) -> String
{
    let mut bits = vec![ vec![0u16;0]; 16] ;

    for i in 0..1u16<<cards.len() {
        let mut bytes : [u8; 2] = [0;2];
        NativeEndian::write_u16(&mut bytes, i);

        let pop_count = weight(&bytes);

        bits[pop_count as usize].push(i);
    }

    let cards:Vec<(char,BigRational)> = 
    cards.into_iter().map( |&(c,n)| (c, BigRational::from(
        BigInt::from(n)))).collect();

    let mut memo : Vec<Option<MemoData>> = vec![ None; 1<<cards.len()];

    let seed = BigRational::from(BigInt::from(S));

    for ( c_idx, c) in cards.iter().enumerate()
    {
        let n = apply_op(c, &seed);
        memo[ 1 << c_idx ] = Some(MemoData {high: n.clone(), low: n.clone()});
    }

    println!("Solving case {}", case_no);

    format!("Case #{}: \n", case_no)
}
