use crate::algo::vectors::*;
use crate::util::codejam::run_cases;
use crate::util::grid::Grid;
use num_bigint::BigInt;
use num_traits::*;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::{SliceRandom, StdRng};
use rand::{Rng, SeedableRng};
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::Write;
use std::ops::Sub;
use std::time::Instant;

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
        &[
            "E-small-practice",
          //  "E-large-practice"
        ],
        "y2017round4",
        |reader, buffer| {
            let P = reader.read_int();
            //suit/value
            let premade_stacks: Vec<Vec<(u16, u16)>> = (0..P)
                .map(|_| {
                    let nums = reader.read_num_line::<u16>();
                    let num_cards = nums[0] as usize;
                    (0..num_cards)
                        .map(|cn| (nums[1 + 2 * cn], nums[2 + 2 * cn]))
                        .collect()
                })
                .collect();

            let t = reader.read_int();

            for case in 1..=t {
                let (N, C) = reader.read_tuple_2::<usize>();
                let stack_indexes = reader.read_num_line::<usize>();
                let stacks = stack_indexes
                    .iter()
                    .map(|si| {
                        assert_eq!(C, premade_stacks[*si].len());
                        premade_stacks[*si].iter().cloned().collect::<VecDeque<_>>()
                    })
                    .collect();

                if case != 1 {
                    //continue;
                }

                println!("Solving {}", case);

                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case,
                    if solve(&stacks) {
                        "POSSIBLE"
                    } else {
                        "IMPOSSIBLE"
                    }
                )
                .unwrap();
            }
        },
    );
}

fn solve(stacks: &Vec<VecDeque<(u16, u16)>>) -> bool
{
    let mut suitToCards: HashMap<u16, Vec<u16>> = HashMap::new();
    let mut kingSuitToStack: HashMap<u16, usize> = HashMap::new();
    let mut aceSuitToStack: HashMap<u16, usize> = HashMap::new();

    for &(value, suit) in stacks.iter().flatten() {
        suitToCards.entry(suit).or_insert(Vec::new()).push(value);
    }

    for cards in suitToCards.values_mut() {
        cards.sort();
    }

    for (idx, stack) in stacks.iter().enumerate() {
        debug!("Before  Stack #{}: {:?}", idx, stack);
        for (card_idx, &(value, suit)) in stack.iter().enumerate() {
            let suitCards = &suitToCards[&suit];
            if card_idx == stack.len()-1 && value == suitCards[suitCards.len() - 1] {
                aceSuitToStack.insert( suit, idx);
            }
            if suitCards.len() > 1 && value == suitCards[suitCards.len() - 2] {
                kingSuitToStack.insert(suit, idx);
            }
        }
    }

    if suitToCards.len() < stacks.len() {
        return true;
    }

    if suitToCards.len() > stacks.len() {
        return false;
    }

    for (idx, s) in stacks.iter().enumerate() {
        debug!("Stack #{}: {:?}", idx, s);
    }

    //stack => suit; these stacks have a unique ace at the bottom
    let vertices: HashMap<usize, u16> = aceSuitToStack
        .iter()
        .map(|(suit, stack_idx)| {
            (*stack_idx, *suit)
        })
        .collect();

    debug!("Vertices {:?}", vertices);

    if vertices.len() == stacks.len() {
        return true;
    }

    //We say that a vertex (suit) s is a source if the ace is the only card in this suit,
    let sources: Vec<u16> = vertices
        .iter()
        .filter(|(_, suit)| suitToCards[suit].len() == 1)
        .map(|(_, suit)| *suit)
        .collect();

    debug!("Sources {:?}", sources);

    // that s is a target if there is another ace (of a different suit) in the stack in which the ace of s is at the bottom
    let target: HashSet<u16> = vertices
        .iter()
        .filter(|&(stack_idx, suit)| {
            stacks[*stack_idx].iter().any(|(search_card, search_suit)| {
                suit != search_suit && suitToCards[search_suit].last().unwrap() == search_card
            })
        })
        .map(|(_, suit)| *suit)
        .collect();

    debug!("Targets: {:?}", target);

    //We add an edge from vertex s1 to a different vertex s2 if the king of s2 is in the stack that has the ace of s1 at the bottom.
    let mut edges: HashMap<u16, Vec<u16>> = HashMap::new();

    for (stack_idx_1, suit_1) in vertices.iter() {
        /*
        We add an edge from vertex s1 to a different vertex s2 if the king of s2 is in the stack that has the ace of s1 at the bottom.
        */
        
        if !kingSuitToStack.contains_key(suit_1) {
            continue;
        }
        let kingStack = kingSuitToStack[suit_1];

        if !vertices.contains_key(&kingStack) {
            continue;
        }

        let vertex_2_suit = vertices[&kingStack];

        edges.entry(*suit_1).or_insert(Vec::new()).push(vertex_2_suit);
            
        
    }

    println!("Starting DFS {}", sources.len());
    for source in sources {
        println!("DFS {}", source);
        if DFS(&edges, &mut HashSet::new(), source, &target) {
            return true;
        }
    }

    false
}

fn DFS(
    edges: &HashMap<u16, Vec<u16>>,
    visited: &mut HashSet<u16>,
    v: u16,
    targets: &HashSet<u16>,
) -> bool
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
