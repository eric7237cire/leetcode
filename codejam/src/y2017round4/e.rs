use num_bigint::BigInt;
use crate::util::codejam::run_cases;
use crate::util::grid::Grid;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::{StdRng, SliceRandom};
use rand::{SeedableRng,Rng};
use std::collections::{HashMap, HashSet,VecDeque};
use std::io::Write;
use std::time::Instant;
use std::ops::{Sub};
use crate::algo::vectors::*;
use num_traits::*;

/*
Cross product / dot product
normal vectors in a plane
Dividing plane
Sphere
integer math 
big ints
*/

pub fn solve_all_cases()
{
    /*
     N, the number of cities visited by K. The next N lines contain three integers Xi, Yi and Zi e
     */

    run_cases(
        &["E-small-practice",],
        "y2017round4",
        |reader, buffer| {
            let P = reader.read_int();
            //suit/value
            let premade_stacks: Vec<Vec<(u16,u16)>> = (0..P).map(|_| {
                    let nums = reader.read_num_line::<u16>();
                    let num_cards = nums[0] as usize;
                    (0..num_cards).map(|cn| (nums[1+2*cn], nums[2+2*cn])).collect()
                }).collect();
                

            let t = reader.read_int();

            for case in 1..=t {
                let (N,C) = reader.read_tuple_2::<usize>();
                let stack_indexes = reader.read_num_line::<usize>();
                let stacks = stack_indexes.iter().map(|si| {
                    assert_eq!(C, premade_stacks[*si].len() );
                    premade_stacks[*si].iter().cloned().collect::<VecDeque<_>>()
                }).collect();

                if case != 1 {
                    //continue;
                }

                writeln!(buffer, "{}", solve(case, &stacks)).unwrap();

                
            }
        },
    );
}


fn solve(case_no: u32, stacks: &Vec<VecDeque<(u16, u16)>>) -> String
{
    let mut suitToCards : HashMap<u16, Vec<u16>> = HashMap::new();
    for &(value,suit) in stacks.iter().flatten() {
        suitToCards.entry(suit).or_insert(Vec::new()).push(value);
    } 

    for cards in suitToCards.values_mut() {
        cards.sort();
    }

    for (idx,s) in stacks.iter().enumerate() {
        debug!("Before  Stack #{}: {:?}", idx, s);
    }


    let mut stacks = stacks.clone();
    //pre processing
    'stack_loop: loop {
        let mut suitToStackIndex: HashMap<u16, usize> = HashMap::new();
        for stack_idx in 0..stacks.len() {
            if let Some( &(value, suit) ) = stacks[stack_idx].front()
            {
                let last_index = *suitToStackIndex.entry(suit).or_insert(stack_idx);
                if last_index == stack_idx {
                    continue;
                }
                if stacks[stack_idx].front().unwrap().0 > stacks[last_index].front().unwrap().0 {
                    stacks[last_index].pop_front();
                } else {
                    stacks[stack_idx].pop_front();
                }
                continue 'stack_loop;

            }
        }

        break;
    }

    for (idx,s) in stacks.iter().enumerate() {
        debug!("Stack #{}: {:?}", idx, s);
    }


    if stacks.iter().all( |s| s.len() <= 1) 
    {
        return format!("Case #{}: POSSIBLE", case_no);
    }

    if !stacks.iter().any( |s| s.len() == 0) { 
    
        return format!("Case #{}: IMPOSSIBLE", case_no)
    }

    

    let vertices: Vec<_> = stacks.iter().enumerate().filter( |&(_, stack)|
    {
        if let Some(&(bot_value, bot_suit)) = stack.back() {
        bot_value == *suitToCards[&bot_suit].last().unwrap()
        } else {false}
    }).map( |(stack_idx, stack)| {
        let (bot_value, bot_suit) = *stack.back().unwrap();
        (stack_idx, bot_suit, bot_value) 
    }
    ).collect();

    debug!("Vertices {:?}", vertices);

    //We say that a vertex (suit) s is a source if the ace is the only card in this suit, 
    let sources:Vec<usize> = vertices.iter().enumerate().filter( |(_, (_, suit, _)) |
        suitToCards[suit].len() == 1
    ).map( | (vertex_index, _) | vertex_index).collect();

    debug!("Sources {:?}", sources);

    // that s is a target if there is another ace (of a different suit) in the stack in which the ace of s is at the bottom
    let target:HashSet<usize> = vertices.iter().enumerate().filter( |(_, (stack_idx, suit, _)) |
        stacks[*stack_idx].iter().any(| (search_card, search_suit) | suit!=search_suit &&

        suitToCards[search_suit].last().unwrap() == search_card)

    ).map( | (vertex_index, _) | vertex_index).collect();

    debug!("Targets: {:?}", target);

    //We add an edge from vertex s1 to a different vertex s2 if the king of s2 is in the stack that has the ace of s1 at the bottom.
    let mut edges : HashMap<usize, Vec<usize>> = HashMap::new();
    
    for v1 in 0..vertices.len() {
        for v2 in 0..v1 {
            let s2 = vertices[v2].1;
            if suitToCards[&s2].len() <= 1 {
                continue;
            }
            let king:u16 = suitToCards[&s2][ suitToCards[&s2].len() - 2 ];
            if stacks[v1].iter().any( | &(suit, value)| suit==s2 && value==king) {
                edges.entry(v1).or_insert(Vec::new()).push(v2);
            }
        }
    }

    for source in sources {
        if DFS(&edges, &mut HashSet::new(), source, &target) {
            return format!("Case #{}: POSSIBLE", case_no);
        }
    }

    format!("Case #{}: IMPOSSIBLE", case_no)

}

fn DFS(edges: &HashMap<usize, Vec<usize>>, visited: &mut HashSet<usize>, v: usize, targets: &HashSet<usize> ) -> bool
{
    if targets.contains(&v) {
        return true;
    }
    visited.insert(v);
    let mut found = false;

    if !edges.contains_key(&v) {
        return false;
    }
    for w in edges[&v].iter() {
        if visited.contains(w) {
            continue;
        }
        found |= DFS(edges, visited, *w, targets);

        if found {
            break;
        }
    }

    found
}