use crate::algo::graph::flow2::Flow;
use crate::algo::prime::sieve::SieveOfAtkin;

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

use byteorder::{ByteOrder, NativeEndian, LittleEndian, BigEndian, WriteBytesExt};

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

                writeln!(buffer, "Case #{}:\n{}", case_no, solve(N, J)).unwrap();
            }
        },
    );
}

fn solve(N: u16, J: u16) -> String
{
    let mut ans : Vec<Vec<u64>> = Vec::new();

    let mut buf = [0; 4];

    let mut sieve = SieveOfAtkin::new(10000);
    sieve.run();
    let primes = sieve.get_results_vec();

    //brute force
    'jamcoin: for jamcoin in (1 + (1 << (N-1))..= ((1u64 << N) - 1)  ).step_by(2) {
        //println!("{:b}  N={}", jamcoin, N);

        let mut num_ks = Vec::new();
        
        //BigEndian::write_u32(&mut buf, jamcoin);
        //let bitvec = BitVec::from_bytes(&buf);

        'next_base: for base in 2..=10 {
            let mut num = 0u64;
            let mut base_mul = 1u64;
            for pos in 0..N {
                num += ((jamcoin >> pos) & 1 ) * base_mul;
                base_mul *= base;
            }
            //println!("In base {}, num is {}, is prime: {}", base, num, primes.contains(& (num as u64)));
            
            for &k in primes.iter() {
                if num % k == 0 {
                    num_ks.push(k);
                    if base == 10 {
                        num_ks.insert(0, num as u64);
                        ans.push(num_ks);
                        if ans.len() == J as usize {
                            break 'jamcoin;
                        } else {
                            continue 'jamcoin;
                        }
                    }
                    continue 'next_base;
                }
            }
            continue 'jamcoin; 
        }

        //println!("{:?}  N={}", bitvec, N);

        
    } 
    
    ans.iter().map( |num_list| num_list.iter().map( |num| num.to_string()).collect::<Vec<_>>().join(" ")).collect::<Vec<_>>().join("\n")
}
