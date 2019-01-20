use crate::util::codejam::run_cases;
use std::collections::HashSet;
use std::io::Write;
use std::mem;
use std::cmp::max;

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
            // "A-large-practice"
        ],
        "y2017round4",
        |reader, buffer| {
            let t = reader.read_int();

            for case in 1..=t {
                let N = reader.read_int();

                let dice: Vec<(u32, u16)> = (0..N)
                    .flat_map(|dIdx| reader.read_num_line().into_iter().map(move |v| (v, dIdx)))
                    .collect();

                write!(buffer, "{}", solve(case, &dice)).unwrap();
            }
        },
    );
}

fn solve(case_no: u32, dice: &Vec<(u32, u16)>) -> String
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
        println!("Processing v {}/{}.  Lens {}, {}", v, dIdx, sequences.len(), sequences_next.len());
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
