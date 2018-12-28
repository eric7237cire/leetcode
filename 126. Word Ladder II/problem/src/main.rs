pub struct Solution {}

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::usize;
//code from https://doc.rust-lang.org/std/collections/binary_heap/

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn vec_idx_to_str(vs: &Vec<usize>, usize_to_str: &Vec<String> ) -> Vec<String> {
    vs.iter().map(|&e| usize_to_str[e].clone()).collect::<Vec<_>>()
}

/// Computes a simplified lex distance
pub fn lex_distance(s1: &str, s2: &str) -> u16
{    
    let mut diff_count = 0;

    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 != c2 {
            diff_count += 1;
        }
    }

    return diff_count + (s1.len() as i32 - s2.len() as i32).abs() as u16;
}

#[test]
fn test_lex_distance() {
    assert_eq!(1, lex_distance("bob", "boa"));
    assert_eq!(3, lex_distance("bob22", "boa"));
    assert_eq!(4, lex_distance("bob", "boa222"));

    // not real lex distance but its ok for our needs
    assert_eq!(4, lex_distance("abcd", "bcd"));
}



impl Solution {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        

        let mut word_list = word_list.clone();

        if word_list.iter().find(|&x| *x == begin_word).is_none() {
            word_list.push(begin_word.clone());
        }

        let mut adj_list: Vec<Vec<usize>> = vec![Vec::new(); word_list.len()];

        if word_list.iter().find(|&x| *x == end_word).is_none() {
            return Vec::new();
        }

        let mut start: Option<usize> = None;
        let mut stop = start;

        for (index1, item1) in word_list.iter().enumerate() {
            for (j, item2) in word_list.iter().skip(index1).enumerate() {
                let index2 = j + index1;
                let letter_diffs = lex_distance(item1, item2);
                if 1 == letter_diffs {
                    //println!("Adding {} {} and {} {}", index1, item1, index2, item2);
                    adj_list[index1].push(index2);
                    adj_list[index2].push(index1);
                }
            }

            if *item1 == begin_word {
                start = Some(index1);
            }
            if *item1 == end_word {
                stop = Some(index1);
            }
        }

        let start = start.unwrap();
        let stop = stop.unwrap();

        // dist[node] = current shortest distance from `start` to `node`
        let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();
        let mut prev: Vec<Vec<usize>> = vec![Vec::new(); word_list.len()];

        let mut heap = BinaryHeap::new();

        // We're at `start`, with a zero cost
        dist[start] = 0;
        heap.push(State {
            cost: 0,
            position: start,
        });

        // Examine the frontier with lower cost nodes first (min-heap)
        while let Some(State { cost, position }) = heap.pop() {
            // Alternatively we could have continued to find all shortest paths
            //if position == goal { return Some(cost); }

            // Important as we may have already found a better way
            if cost > dist[position] {
                continue;
            }

            assert_eq!(cost, dist[position]);

            // For each node we can reach, see if we can find a way with
            // a lower cost going through this node
            for neighbor_node in &adj_list[position] {

                let next = State {
                    cost: cost + 1,
                    position: *neighbor_node,
                };

                // If so, add it to the frontier and continue
                // We want all paths, so == cost is OK
                if next.cost <= dist[next.position] {
                    heap.push(next);
                    // Relaxation, we have now found a better way
                    dist[next.position] = next.cost;

                    // Update prev
                    if !prev[next.position].is_empty() {
                        //all costs in prev should be ==
                        let p_cost = dist[ prev[next.position][0] ];
                        if cost < p_cost {
                            println!("Clearing next position {} / {}, was {:?}", 
                                next.position,
                                word_list[next.position].clone(),
                                vec_idx_to_str(&prev[next.position], &word_list)
                            );
                            prev[next.position].clear();
                        } else {
                            assert!(cost <= p_cost);
                        }
                    }
                    if !prev[next.position].contains(&position) {
                        prev[next.position].push(position);
                    }
                }
            }
        }

/*
        println!("distance from start {} to end {} is {}", start, stop, dist[stop]);
        println!("Adj list is {:?}", adj_list);
        for (i, item) in prev.iter().enumerate() {
            println!("Prev for node# {} / {} is {:?}", 
                i,
                word_list[i].clone(),
                vec_idx_to_str(&prev[i], &word_list)
            );
            println!("Dist for node# {} / {} is {:?}", 
                i,
                word_list[i].clone(),
                dist[i]
            );
        }
*/
        if dist[stop] == usize::MAX {
            return Vec::new();
        }

        //the +1 is because the distance is edges and we want # of nodes
        let min_sol_length = dist[stop] + 1;
        
        let mut all_paths : Vec<Vec<usize>> = Vec::new();

        //generate all_paths from the prev lists
        all_paths.push( vec![stop] );

        while all_paths[0].len() < min_sol_length
        {
            let all_path_cur_len = all_paths.len();

            for all_path_index in 0..all_path_cur_len
            {
                let last_node:usize;
                {
                    //This adds on the 1st prev node to the end of the path
                    let path : &mut Vec<usize> = &mut all_paths[all_path_index];
                    last_node = *path.last().unwrap();                    
                    path.push( prev[last_node][0] );
                }

                //if we had more than 1 choice, we need to clone new paths 
                for prev_node in prev[last_node].iter().skip(1)
                {                    
                    let mut new_path = all_paths[all_path_index].clone();
                    //Since we added a node already above
                    *new_path.last_mut().unwrap() = *prev_node ;
                    all_paths.push(new_path);                    
                }
                
            }
        }

        //convert nodes to strings
        let mut all_path_strings: Vec<Vec<String>> = Vec::new();

        for p in all_paths.iter() {
            let ps = p.iter().rev().map( |n| word_list[*n].clone()).collect::<Vec<String>>();
            all_path_strings.push(ps);
        }

        return all_path_strings;
    }
}

fn main() {
    let checks: [((&'static str, &'static str, Vec<&str>), Vec<Vec<&str>>); 3] = [(
        ("hit", "cog", vec!["hot", "dot", "dog", "lot", "log", "cog"]),
        vec![
            vec!["hit", "hot", "lot", "log", "cog"],
            vec!["hit", "hot", "dot", "dog", "cog"],
            
        ],
    ),
    
 (   ("hot",
    "dog",
vec!["hot","dog"]),
vec![]
),

(( "red",
"tax",
vec!["ted","tex","red","tax","tad","den","rex","pee"]),

vec![
    vec!["red","ted","tad","tax"],
    vec!["red","rex","tex","tax"],
    vec!["red","ted","tex","tax"]
    
    ]
)
    ];

    println!("Hello, world!");

    for check in checks.iter() {
        let solution_args = &check.0;
        let mut expected_ans: Vec<Vec<String>> = Vec::new();
        for v1 in check.1.iter() {
            expected_ans.push(v1.iter().map(|&e| e.to_string()).collect::<Vec<_>>());
        }
        let actual_ans = Solution::find_ladders(
            solution_args.0.to_string(),
            solution_args.1.to_string(),
            solution_args
                .2
                .iter()
                .map(|&e| e.to_string())
                .collect::<Vec<String>>(),
        );
        if expected_ans != actual_ans {
            println!("Problem {:?} != {:?}", actual_ans, expected_ans);
        } else {
            println!("OK {:?} == {:?}", actual_ans, expected_ans);
        }
        //break;
    }
}
