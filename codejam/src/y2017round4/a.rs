use crate::algo::graph::flow2::Flow;
use crate::util::codejam::run_cases;
use bit_set::BitSet;
use rand::{thread_rng, Rng};
use std::cmp::max;
use std::collections::HashSet;
use std::collections::VecDeque;
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
            // "A-large-practice"
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

                if case != 3 {
                    continue;
                }

                write!(buffer, "{}", solve(case, &dice)).unwrap();
            }
        },
    );
}

const NUM_DICE_VALUES: usize = 6;
const MAX_DICE_VALUE: usize = 1_000_001;
const MAX_N_DICE: usize = 50_000;
/*
fn bfs(v: i32 was: &mut Vec<i32>,  pb: &mut Vec<i32>, pa: &mut Vec<i32>, value_to_dice: &Vec<Vec<usize>>, iter: i32) -> bool{


        // Mark all the vertices as not visited
        let mut visited = Bitset::new();

        //# Create a queue for BFS

        let mut queue: VecDeque<usize> = VecDeque::new();

        //# Mark the source node as visited and enqueue it
        queue.append(s)
        visited[s] = True

        # Standard BFS Loop
        while queue:
            u = queue.popleft()

            # Get all adjacent vertices's of the dequeued vertex u
            # If a adjacent has not been visited, then mark it
            # visited and enqueue it
            for ind, val in enumerate(self.graph[u]):
                if (visited[ind] == False) and (val > 0):
                    queue.append(ind)
                    visited[ind] = True
                    parent[ind] = u

        # If we reached sink in BFS starting from source, then return
        # true, else false
        return visited[t]
*/
fn dfs(
    v: i32,
    was: &mut Vec<i32>,
    pb: &mut Vec<i32>,
    pa: &mut Vec<i32>,
    value_to_dice: &Vec<Vec<usize>>,
    iter: i32,
) -> bool
{
    //pb is dice value=>dice index
    //pa is dice index=>dice value
    was[v as usize] = iter;

    //free matching
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

fn add_value_to_flow(flow: &mut Flow, value_to_add: usize, value_to_dice: &Vec<Vec<usize>>)
{
    flow.add_edge(flow.source, value_to_add, 1);
    for d_idx in value_to_dice[value_to_add].iter() {
        flow.add_edge(value_to_add, MAX_DICE_VALUE + d_idx, 1);
    }

    debug!("After adding value {}", value_to_add);

    debug_print_flow(flow);
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
        .find(| &&edge_index | {
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

    for &edge_idx in flow.V[value_to_remove].iter() {
        //deleting the edge, this value no longer can be matched
        {
            let flow_rev_edge = &mut flow.E[edge_idx ^ 1];
            flow_rev_edge.cap = 0;
            flow_rev_edge.residue = 0;
        }
        {
            let flow_edge = &mut flow.E[edge_idx];
            flow_edge.cap = 0;
            flow_edge.residue = 0;

            //this is the source->left hand side edge
            if edge_idx % 2 == 1 {
                assert_eq!(flow_edge.dest, flow.source);
                continue;
            }
        }
    }

    debug!("After removing value {}", value_to_remove);

    debug_print_flow(flow);
}

fn solve(case_no: u32, dice: &Vec<Vec<i32>>) -> String
{
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

    let mut flow = Flow::new(source, sink);

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
    while let Some(val) = it.next() {
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

fn solve2(case_no: u32, dice: &Vec<Vec<i32>>) -> String
{
    let mut value_to_dice: Vec<Vec<usize>> = vec![vec![]; MAX_DICE_VALUE + 1];
    for (didx, d) in dice.iter().enumerate() {
        for d_pos in 0..NUM_DICE_VALUES {
            value_to_dice[d[d_pos] as usize].push(didx);
        }
    }

    for vec in value_to_dice.iter_mut() {
        thread_rng().shuffle(vec);
    }

    let mut pa: Vec<i32> = vec![-1; MAX_DICE_VALUE];
    let mut was: Vec<i32> = vec![-1; MAX_DICE_VALUE];
    let mut pb: Vec<i32> = vec![-1; dice.len()];

    let mut ans = 0;
    let mut rr = 0i32;
    let mut iter = 0i32;
    for ll in 1..MAX_DICE_VALUE {
        rr = max(rr, ll as i32 - 1);
        loop {
            iter += 1;
            if dfs(
                rr as i32 + 1,
                &mut was,
                &mut pb,
                &mut pa,
                &value_to_dice,
                iter,
            ) {
                rr += 1;
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
