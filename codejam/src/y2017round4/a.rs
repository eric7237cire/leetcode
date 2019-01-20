use crate::util::codejam::run_cases;
use std::cmp::max;
use std::collections::HashSet;
use std::io::Write;
use std::mem;

/*
permutations with repeated elements
digit manipulation
recursion
*/
pub fn solve_all_cases()
{
    run_cases(
        &[
            "A-small-practice",
             "A-large-practice"
        ],
        "y2017round4",
        |reader, buffer| {
            let t = reader.read_int();

            for case in 1..=t {
                let N = reader.read_int();

                let dice: Vec<Vec<i32>> = (0..N)
                    .map(|_| reader.read_num_line())
                    //.map(|dIdx| reader.read_num_line().into_iter().map(move |v| (v, dIdx)))
                    .collect();

                write!(buffer, "{}", solve(case, &dice)).unwrap();
            }
        },
    );
}

const NUM_DICE_VALUES: usize = 6;
const MAX_DICE_VALUE: usize = 1_000_000;


fn dfs(v: i32, was: &mut Vec<i32>,  pb: &mut Vec<i32>, pa: &mut Vec<i32>, value_to_dice: &Vec<Vec<usize>>, iter: i32) -> bool{
  was[v as usize] = iter;
  for &j in value_to_dice[v as usize].iter() {
    if pb[j as usize] == -1 {
      pa[v as usize] = j as i32;
      pb[j as usize] = v;
      return true;
    }
  }
  for &j in value_to_dice[v as usize].iter() {
    if was[pb[j] as usize] != iter {
      if dfs(pb[j], was, pb, pa, value_to_dice, iter) {
        pa[v as usize] = j as i32;
        pb[j as usize] = v;
        return true;
      }
    }
  }
  return false;
}

use rand::{thread_rng, Rng};

fn solve(case_no: u32, dice: &Vec<Vec<i32>>) -> String
{
    let mut value_to_dice: Vec<Vec<usize>> = vec![vec![]; MAX_DICE_VALUE + 1];
    for (didx, d) in dice.iter().enumerate() {
        for d_pos in 0..NUM_DICE_VALUES {
            value_to_dice[d[d_pos] as usize].push(didx);
        }
    }

    for vec in value_to_dice.iter_mut()
    {
        thread_rng().shuffle(vec);
    }

    let mut pa : Vec<i32> = vec![-1; MAX_DICE_VALUE];
    let mut was: Vec<i32> = vec![-1; MAX_DICE_VALUE];
    let mut pb : Vec<i32> = vec![-1; dice.len()];
      
    let mut ans = 0;
    let mut rr = 0i32;
    let mut iter = 0i32;
    for ll in 1..MAX_DICE_VALUE {
      rr = max(rr, ll as i32 - 1);
      loop {
        iter+=1;
        if dfs(rr as i32 + 1, &mut was, &mut pb, &mut pa, &value_to_dice, iter) {
          rr+=1;
        } else {
          break;
        }
      }
      ans = max(ans, rr - ll as i32 + 1);
      if pa[ll] != -1 {
        pb[pa[ll as usize] as usize] = -1;
        pa[ll] = -1;
      }
    }
   

    format!("Case #{}: {}\n", case_no, ans)
}

fn solve_brute_force(case_no: u32, dice: &Vec<(u32, u16)>) -> String
{
    let mut all_values = dice.clone();
    let mut longest = 0;
    let mut sequences: Vec<HashSet<u16>> = Vec::new();
    let mut sequences_next: Vec<HashSet<u16>> = Vec::new();
    //dbg!(dice.iter());
    all_values.sort();
    let mut last_value = 0;

    println!("case {}", case_no);

    let mut v_it = all_values.into_iter().peekable();
    while let Some((v, dIdx)) = v_it.next() {
        println!(
            "Processing v {}/{}.  Lens {}, {}",
            v,
            dIdx,
            sequences.len(),
            sequences_next.len()
        );
        if v > last_value + 1 {
            sequences.clear();
            last_value = v - 1;
        }
        for seq in sequences.iter() {
            if !seq.contains(&dIdx) {
                let mut s = seq.clone();
                s.insert(dIdx);
                longest = max(s.len(), longest);
                sequences_next.push(s);
            }
        }
        let mut h = HashSet::new();
        h.insert(dIdx);
        sequences_next.push(h);

        if let Some(&(v_next, _)) = v_it.peek() {
            if v_next > v {
                sequences.clear();
                mem::swap(&mut sequences, &mut sequences_next);
                last_value = v;
            }
        }
    }

    format!("Case #{}: {}\n", case_no, longest)
}
