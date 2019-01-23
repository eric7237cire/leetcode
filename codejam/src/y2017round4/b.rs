use crate::algo::graph::flow2::Flow;
use crate::util::codejam::run_cases;
use bit_set::BitSet;
use bit_vec::BitVec;
use rand::{thread_rng, Rng};
use std::cmp::{max, min};
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::Write;
use std::mem;
use std::usize;

use byteorder::{ByteOrder, NativeEndian, WriteBytesExt};
use hamming::weight;
use num_bigint::BigInt;
use num_integer::Integer;
use num_rational::{BigRational, Ratio};
use num_traits::{FromPrimitive, One, Signed, Zero};
use std::ops::{Add, Div, Mul, Sub};
use std::thread;

/*
Dynamic programming, min/max
Arithmetic
Grouping terms
*/
pub fn solve_all_cases()
{
    run_cases(
        &["B-small-practice", "B-large-practice"],
        "y2017round4",
        |reader, buffer| {
            let t = reader.read_int();

            for case in 1..=t {
                let (S, C) = reader.read_tuple_2::<i16>();

                let cards = (0..C)
                    .map(|_| {
                        let mut sw = reader.read_string().split_whitespace();
                        (
                            sw.next().unwrap().parse::<char>().unwrap(),
                            sw.next().unwrap().parse::<i16>().unwrap(),
                        )
                    })
                    .collect();

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
    match card.0 {
        '+' => num + &card.1,
        '-' => num - &card.1,
        '*' => num * &card.1,
        '/' => num / &card.1,
        _ => {
            assert!(false);
            num * BigRational::from_i8(1).unwrap()
        }
    }
}

fn solve(case_no: u32, cards: &Vec<(char, i16)>, S: i16) -> String
{
    println!("Solving {}", case_no);

    let mut bits = vec![vec![0u16; 0]; 16];

    let mut cards: Vec<(char, BigRational)> = cards
        .into_iter()
        .map(|&(c, n)| (c, BigRational::from(BigInt::from(n))))
        .collect();

    let add_card: BigRational = cards
        .iter()
        .filter(|(op, val)| {
            (*op == '+' && val >= &BigRational::zero())
                || (*op == '-' && val <= &BigRational::zero())
        })
        .fold(BigRational::zero(), |acc, (op, val)| &acc + val.abs());

    let sub_card: BigRational = cards
        .iter()
        .filter(|(op, val)| {
            (*op == '+' && val < &BigRational::zero()) || (*op == '-' && val > &BigRational::zero())
        })
        .fold(BigRational::zero(), |acc, (op, val)| &acc + val.abs());

    let has_mul_zero = cards
        .iter()
        .any(|(op, val)| (*op == '*' && val == &BigRational::zero()));

    let mut neg_mul_cards: Vec<_> = cards
        .iter()
        .filter(|(op, val)| (*op == '*' && val < &BigRational::zero()))
        .cloned()
        .collect();
    neg_mul_cards.sort();
    neg_mul_cards.reverse();

    let mut neg_div_cards: Vec<_> = cards
        .iter()
        .filter(|(op, val)| (*op == '/' && val < &BigRational::zero()))
        .cloned()
        .collect();
    neg_div_cards.sort();
    neg_div_cards.reverse();

    let mul_pos_card: BigRational = cards
        .iter()
        .filter(|(op, val)| *op == '*' && val > &BigRational::zero())
        .fold(BigRational::one(), |acc, (op, val)| &acc * val);

    let div_pos_card: BigRational = cards
        .iter()
        .filter(|(op, val)| *op == '/' && val > &BigRational::zero())
        .fold(BigRational::one(), |acc, (op, val)| &acc * val);

    cards.clear();

    if add_card != BigRational::zero() {
        cards.push(('+', add_card));
    }
    if sub_card != BigRational::zero() {
        cards.push(('-', sub_card));
    }

    for (_, neg_val) in neg_mul_cards.iter().take(2) {
        cards.push(('*', neg_val.clone()));
    }

    if neg_mul_cards.len() >= 3 {
        let mul_neg_val: BigRational = neg_mul_cards
            .iter()
            .skip(2)
            .fold(BigRational::one(), |acc, (op, val)| &acc * val);
        cards.push(('*', mul_neg_val));
    }

    if mul_pos_card != BigRational::one() {
        cards.push(('*', mul_pos_card));
    }
    if has_mul_zero {
        cards.push(('*', BigRational::from_u16(0).unwrap()));
    }

    for (_, neg_val) in neg_div_cards.iter().take(2) {
        cards.push(('/', neg_val.clone()));
    }

    if neg_div_cards.len() >= 3 {
        let div_neg_val: BigRational = neg_div_cards
            .iter()
            .skip(2)
            .fold(BigRational::one(), |acc, (op, val)| &acc * val);
        cards.push(('/', div_neg_val));
    }

    if div_pos_card != BigRational::one() {
        cards.push(('/', div_pos_card));
    }

    assert!(cards.len() <= 15);

    if cards.len() == 0 {
        return format!("Case #{}: 0 1\n", case_no);
    }

    //create subsets for each pop_count
    for i in 0..1u16 << cards.len() {
        let mut bytes: [u8; 2] = [0; 2];
        NativeEndian::write_u16(&mut bytes, i);

        let pop_count = weight(&bytes);

        bits[pop_count as usize].push(i);
    }

    let mut memo: Vec<Option<MemoData>> = vec![None; 1 << cards.len()];

    let seed = BigRational::from(BigInt::from(S));

    for (c_idx, c) in cards.iter().enumerate() {
        let n = apply_op(c, &seed);
        memo[1 << c_idx] = Some(MemoData {
            high: n.clone(),
            low: n.clone(),
        });
    }

    for level in 2..=cards.len() {
        debug!("Looking at level {}.  Size: {}", level, bits[level].len());
        for &perm in bits[level].iter() {
            //Basically we need to calculate min/max based on which one we apply last
            //so the perm will be [1....1....1] with bits set on the cards in the perm
            //we have already calculated all comibations with one less bit
            let mut cur_min = None;
            let mut cur_max = None;

            for (c_idx, c) in cards.iter().enumerate() {
                //https://stackoverflow.com/questions/47981/how-do-you-set-clear-and-toggle-a-single-bit/50691
                if 1 & (perm >> c_idx) == 0 {
                    continue;
                }

                //find mins / maxes when starting with this bit unset
                let other_perm = perm & !(1 << c_idx);
                let other_min_max = memo[other_perm as usize].as_ref().unwrap();

                let mut new_min = apply_op(&c, &other_min_max.low);
                let mut new_max = apply_op(&c, &other_min_max.high);

                if c.1.is_negative() && (c.0 == '*' || c.0 == '/') {
                    mem::swap(&mut new_min, &mut new_max);
                }

                if let Some(cur_min_uw) = cur_min.as_ref() {
                    if &new_min < cur_min_uw {
                        cur_min = Some(new_min);
                    }
                } else {
                    cur_min = Some(new_min);
                }
                /*
                cur_min = Some( {
                    min(cur_min, new_min)
                } else {
                    new_min
                });*/
                cur_max = Some(if let Some(cur_max) = cur_max {
                    max(cur_max, new_max)
                } else {
                    new_max
                });
            }
            memo[perm as usize] = Some(MemoData {
                high: cur_max.unwrap(),
                low: cur_min.unwrap(),
            });
        }
    }

    let ans = &memo[(1 << cards.len()) - 1].as_ref().unwrap().high;

    format!("Case #{}: {} {}\n", case_no, ans.numer(), ans.denom())
}
