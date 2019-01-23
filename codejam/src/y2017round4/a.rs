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
incremental matching
performance intentive
heavy duty recursion (in slower solutions)

Ideas: Parallel DFS?  Faster matching algorithm/implementation?
Hopcroft–Karp_algorithm?

Make flow2 faster (which is also BFS but way slower than the solution used)

Found a fast BFS implementation which is currently used in solve from
one of the contestents
*/
pub fn solve_all_cases()
{
    run_cases(
        &["A-small-practice", "A-large-practice"],
        "y2017round4",
        |reader, buffer| {
            let t = reader.read_int();

            for case in 1..=t {
                let N = reader.read_int();

                let dice: Vec<Vec<i32>> = (0..N)
                    .map(|_| reader.read_num_line())
                    //.map(|dIdx| reader.read_num_line().into_iter().map(move |v| (v, dIdx)))
                    .collect();

                if case != 3 {
                    // continue;
                }

                let child = thread::Builder::new()
                    .stack_size(STACK_SIZE)
                    .spawn(move || solve(case, &dice))
                    .unwrap();

                // Wait for thread to join
                let ans = child.join().unwrap();

                write!(buffer, "{}", ans).unwrap();
            }
        },
    );
}

const STACK_SIZE: usize = 40 * 1024 * 1024;

const NUM_DICE_VALUES: usize = 6;
const MAX_DICE_VALUE: usize = 1_000_000;
const MAX_N_DICE: usize = 50_000;

const INVALID_MATCH_i32: i32 = -1;

fn solve(case_no: u32, dice: &Vec<Vec<i32>>) -> String
{
    println!("Solving case {}", case_no);

    let mut value_to_dice: Vec<Vec<usize>> = vec![vec![]; MAX_DICE_VALUE + 1];
    for (didx, dice_values) in dice.iter().enumerate() {
        for d_value in dice_values.iter() {
            value_to_dice[*d_value as usize].push(didx);
        }
    }

    let n = dice.len();

    //matchL[dice value]
    let mut matchL = vec![INVALID_MATCH_i32; MAX_DICE_VALUE];

    //matchR[dice index]
    let mut matchR = vec![INVALID_MATCH_i32; n];

    //Storing values
    let mut queue = vec![0; MAX_DICE_VALUE];
    let mut back = vec![0; MAX_DICE_VALUE];

    let mut used = BitVec::from_elem(MAX_DICE_VALUE, false);
    let mut ans = 0;
    let mut rangeStart = 0;
    //looks like [rangeStart, rangeEnd)
    let mut rangeEnd = 0;
    let mut cur_dice_value_i = 0;
    while cur_dice_value_i < MAX_DICE_VALUE {
        if value_to_dice[cur_dice_value_i].len() == 0 {
            //				System.err.println(i + " NOEDGE");
            for j in rangeStart..rangeEnd {
                matchR[matchL[j] as usize] = INVALID_MATCH_i32;
                matchL[j] = INVALID_MATCH_i32;
            }
            rangeStart = cur_dice_value_i + 1;
            rangeEnd = cur_dice_value_i + 1;
            cur_dice_value_i += 1;
            continue;
        }
        let mut queueHead = 0;
        let mut queueTail = 1;
        queue[0] = cur_dice_value_i as i32;
        used.set(cur_dice_value_i, true);
        back[cur_dice_value_i] = INVALID_MATCH_i32;
        let mut found = false;
        'bfs: loop {
            assert!(queue[queueHead] >= 0);
            let mut cur = queue[queueHead] as usize;
            queueHead += 1;

            let mut cedges = &value_to_dice[cur];
            for j in 0..cedges.len() {
                let mut next_dice_index = cedges[j] as usize;
                //Found a non matched dice index
                if matchR[next_dice_index] < 0 {
                    matchR[next_dice_index] = cur as i32;
                    //Applying the augmenting path
                    while back[cur] >= 0 {
                        assert!(back[cur] >= 0);
                        assert!(matchL[cur] >= 0);
                        let prev = back[cur] as usize;
                        let pnext = matchL[cur] as usize;
                        matchL[cur] = next_dice_index as i32;
                        matchR[pnext] = prev as i32;
                        cur = prev;
                        next_dice_index = pnext;
                    }
                    matchL[cur] = next_dice_index as i32;
                    found = true;
                    break 'bfs;
                } else if (!used[matchR[next_dice_index] as usize]) {
                    //Need to find a new matching for this value, put its dice value on queue
                    used.set(matchR[next_dice_index] as usize, true);
                    queue[queueTail] = matchR[next_dice_index];
                    queueTail += 1;

                    back[matchR[next_dice_index] as usize] = cur as i32;
                }
            }
            if queueHead == queueTail {
                break;
            }
        }
        //			System.err.println(Arrays.toString(matchL));
        //			System.err.println(Arrays.toString(matchR));
        if (!found) {
            //				System.err.println(i + " NOFOUND");
            while (true) {
                assert_ne!(rangeStart, rangeEnd);
                //Reset dice index
                matchR[matchL[rangeStart] as usize] = INVALID_MATCH_i32;
                matchL[rangeStart] = INVALID_MATCH_i32;
                assert_ne!(rangeStart, rangeEnd);
                rangeStart += 1;

                //Not sure what this is for
                if (used[rangeStart - 1]) {
                    //						System.err.println("ADJ " + rangeStart);
                    break;
                }
            }
            cur_dice_value_i -= 1;
        } else {
            //				System.err.println(i + " FOUND");
            rangeEnd += 1;
            ans = max(ans, rangeEnd - rangeStart);
        }

        //Reset all dice values in queue
        for j in 0..queueTail {
            used.set(queue[j] as usize, false);
        }

        cur_dice_value_i += 1;
    }

    format!("Case #{}: {}\n", case_no, ans)
}

const INVALID_MATCH: usize = usize::MAX - 1;

struct DfsDice
{
    //vis[dice value] = true if already visited
    vis: BitVec,
    //mat[dice index] = dice value
    mat: Vec<usize>,
    //e[dice value] = vec of dice indexes with that value
}

impl DfsDice
{
    /// Conceptually, the dice values are on the LHS and
    /// dice indicies are RHS
    /// mat[dice index] = value is an edge in the matching
    fn dfs(&mut self, dice_value: usize, e: &Vec<Vec<usize>>) -> bool
    {
        self.vis.set(dice_value, true);
        //Any free matchings?
        for &dice_index in e[dice_value].iter() {
            if self.mat[dice_index] == INVALID_MATCH {
                self.mat[dice_index] = dice_value;
                return true;
            }
        }

        for &dice_index in e[dice_value].iter() {
            //ignore if matched dice value (LHS) is aready visited
            //2nd part is to search the already matched value to find an augmenting path
            if !self.vis[self.mat[dice_index]] && self.dfs(self.mat[dice_index], e) {
                self.mat[dice_index] = dice_value;
                return true;
            }
        }

        return false;
    }
}

fn add_value_to_flow(flow: &mut Flow, value_to_add: usize, value_to_dice: &Vec<Vec<usize>>)
{
    flow.add_edge(flow.source, value_to_add, 1);
    for d_idx in value_to_dice[value_to_add].iter() {
        flow.add_edge(value_to_add, MAX_DICE_VALUE + d_idx, 1);
    }

    debug!("After adding value {}", value_to_add);

    //debug_print_flow(flow);
}

fn debug_print_flow(flow: &Flow)
{
    for (idx, edge) in flow.E.iter().enumerate() {
        if idx % 2 == 0 && edge.residue < edge.cap {
            debug!(
                "Flow {} / {} flow at node {}->node {} edge idx {} \n",
                edge.cap - edge.residue,
                edge.cap,
                edge.src,
                edge.dest,
                idx
            );
        }

        assert_eq!(flow.E[idx ^ 1].residue + edge.residue, edge.cap);
        assert_eq!(flow.E[idx ^ 1].cap, edge.cap);
    }
}

fn remove_value_from_flow(flow: &mut Flow, value_to_remove: usize)
{
    //assert_eq!(flow.V[interval_start].len(), 1);

    //find the matching dice index
    let matching_edge_index: usize = flow.V[value_to_remove]
        .iter()
        .find(|&&edge_index| {
            edge_index % 2 == 0 && flow.E[edge_index].cap > 0 && flow.E[edge_index].residue == 0
        })
        .map(|ei| *ei)
        .unwrap();

    flow.reset_edge_flow(matching_edge_index);

    //let matching_edge = &flow.E[matching_edge_index];

    let dice_vertex = flow.E[matching_edge_index].dest;

    //Find the dice->sink edge
    let dice_sink_edge_index = flow.V[dice_vertex]
        .iter()
        .enumerate()
        .find(|(idx, &edge_index)| {
            idx % 2 == 0 && flow.E[edge_index].cap > 0 && flow.E[edge_index].residue == 0
        })
        .map(|(_, ei)| *ei)
        .unwrap();

    //let dice_sink_edge = &flow.E[dice_sink_edge_index];

    assert_eq!(flow.E[dice_sink_edge_index].dest, flow.sink);

    flow.reset_edge_flow(dice_sink_edge_index);

    //edge connected lhs value to a dice with a face
    //containing that value in right hand set of the
    //matching

    {
        let matching_edge = &flow.E[matching_edge_index];

        assert_eq!(matching_edge.src, value_to_remove);
        //used the scheme that RHS vertexes are assigned
        //MAX_DICE_VALUE + (dice index)
        assert!(
            matching_edge.dest >= MAX_DICE_VALUE
                && matching_edge.dest < (MAX_DICE_VALUE + MAX_N_DICE),
            format!(
                "dest node {} not in range of dice [{}, {})",
                matching_edge.dest,
                MAX_DICE_VALUE,
                MAX_DICE_VALUE + MAX_N_DICE
            )
        );
    }

    let edges_to_remove: Vec<_> = flow.V[value_to_remove].iter().cloned().collect();
    for edge_idx in edges_to_remove {
        //deleting the edge, this value no longer can be matched
        flow.remove_edge(edge_idx);

        //this is the source->left hand side edge
        if edge_idx % 2 == 1 {
            assert_eq!(flow.E[edge_idx].dest, flow.source);
        }
    }
    flow.V[value_to_remove].clear();

    debug!("After removing value {}", value_to_remove);

    //debug_print_flow(flow);
}

/// Very elegant DFS solution, a bit slow though ~2 minutes
fn solve4(case_no: u32, dice: &Vec<Vec<i32>>) -> String
{
    println!("Solving case {}", case_no);

    let mut value_to_dice: Vec<Vec<usize>> = vec![vec![]; MAX_DICE_VALUE + 1];
    for (didx, dice_values) in dice.iter().enumerate() {
        for d_value in dice_values.iter() {
            value_to_dice[*d_value as usize].push(didx);
        }
    }

    //node schema
    //dice indexes are (MAX_DICE_VALUE + N_MAX]
    let mut dfsDice = DfsDice {
        //e: value_to_dice,
        vis: BitVec::from_elem(MAX_DICE_VALUE + 1, false),
        mat: vec![INVALID_MATCH; MAX_DICE_VALUE + 1],
    };

    let mut interval_start = 1;
    let mut interval_stop = 1;
    //interval is [interval_start, interval_stop)
    let n = dice.len();
    let mut ans = 0;

    while interval_stop <= MAX_DICE_VALUE {
        //Anything in the interval is free to be rematched
        for i in interval_start..=interval_stop {
            dfsDice.vis.set(i, false);
        }
        if dfsDice.dfs(interval_stop, &value_to_dice) {
            interval_stop += 1;
            ans = max(ans, interval_stop - interval_start);
        } else {
            for i in 0..n {
                if dfsDice.mat[i] == interval_start {
                    dfsDice.mat[i] = INVALID_MATCH;
                }
            }
            interval_start += 1;
            interval_stop = max(interval_start, interval_stop);
        }
    }

    format!("Case #{}: {}\n", case_no, ans)
}

/// My solution, too slow for large, the augment takes too long
fn solve3(case_no: u32, dice: &Vec<Vec<i32>>) -> String
{
    println!("Solving case {}", case_no);
    let mut unique_dice_values: Vec<i32> = Vec::new();

    let mut value_to_dice: Vec<Vec<usize>> = vec![vec![]; MAX_DICE_VALUE + 1];
    for (didx, dice_values) in dice.iter().enumerate() {
        for d_value in dice_values.iter() {
            value_to_dice[*d_value as usize].push(didx);
            unique_dice_values.push(*d_value);
        }
    }

    unique_dice_values.sort();
    unique_dice_values.dedup();

    //bipartite matching, left side are dice values, right side are dice

    //node schema
    //dice indexes are (MAX_DICE_VALUE + N_MAX]
    let source = MAX_DICE_VALUE + MAX_N_DICE + 1;
    let sink = MAX_DICE_VALUE + MAX_N_DICE + 2;

    let mut flow = Flow::new(source, sink, sink + 1);

    //inclusive range
    let mut interval_start = unique_dice_values[0] as usize;
    let mut interval_stop = unique_dice_values[0] as usize;

    for d_idx in 0..dice.len() {
        flow.add_edge(MAX_DICE_VALUE + d_idx, flow.sink, 1);
    }

    add_value_to_flow(&mut flow, unique_dice_values[0] as usize, &value_to_dice);
    assert!(flow.augment() > 0);

    let mut ans = 0;
    let mut last_val = unique_dice_values[0] as usize;

    let mut it = unique_dice_values.into_iter().peekable();
    it.next();
    /*
            4 8 15 16 23 42
        8 6 7 5 30 9
        1 2 3 4 55 6
        2 10 18 36 54 86


    1 2 3 4 5 6
    1 2 3 4 5 6
    1 4 2 6 5 3
        */

    let mut counter = 0;

    while let Some(val) = it.next() {
        counter += 1;
        if counter % 100 == 0 {
            println!(
                "Loop count {}.  Num graph edges {} Sink edges: {} Source edges: {} 
            Interval start {} stop {}
            ",
                counter,
                flow.E.len(),
                flow.V[flow.sink].len(),
                flow.V[flow.source].len(),
                interval_start,
                interval_stop
            );
        }
        let val = val as usize;
        add_value_to_flow(&mut flow, val, &value_to_dice);

        loop {
            if flow.augment() > 0 {
                //assert_eq!(interval_stop, val - 1);
                interval_stop = val;

                break;
            } else {
                assert!(interval_start < val);
                //flow.setIgnoreNode(interval_start, true);
                //a die

                remove_value_from_flow(&mut flow, interval_start);
                interval_start += 1;
            }
        }

        if val > last_val + 1 {
            for v in interval_start..=last_val {
                remove_value_from_flow(&mut flow, v);
            }

            interval_start = val;
            assert_eq!(interval_stop, val);
        }

        ans = max(ans, interval_stop - interval_start + 1);
        last_val = val;
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
