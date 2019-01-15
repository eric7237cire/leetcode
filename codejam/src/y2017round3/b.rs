use super::super::util::input::*;
//use crate::algo::graph::cycles::simple_cycles;
//use crate::algo::graph::scc::strongly_connected_components;
use crate::algo::graph::DiGraph;
//use std::collections::HashMap;
use bit_set::BitSet;
use bit_vec::BitVec;
//use std::collections::HashMap;
use std::io::Write;
use std::time::Instant;

/*

*/
pub fn solve_all_cases()
{
    let now = Instant::now();

    let mut reader = InputReader::new();
    let t = reader.read_int();

    for case in 1..=t {
        let (F, P) = reader.read_tuple_2();
        let mut G = DiGraph::new();
        for f in 1..=F {
            G.add_vertex(f);
        }
        let P = (0..P)
            .map(|_| {
                let (f1, f2) = reader.read_tuple_2();
                G.add_edge(f1, f2);
                (f1, f2)
            })
            .collect::<Vec<_>>();

        print!("{}", solve(case, &G, &P, F));
    }

    let duration = now.elapsed();
    let secs = f64::from(duration.as_secs() as u32) + f64::from(duration.subsec_nanos()) / 1e9f64;
    let _ = writeln!(
        ::std::io::stderr(),
        "\nElapsed time {:.2} second(s)\n",
        secs
    );
}

fn solve(case_no: u32, G: &DiGraph, P: &Vec<(usize, usize)>, F: usize) -> String
{
    debug!("\n\n\nSolving case {}", case_no);

    let mut g_undirected = G.clone();
    for (u, v) in g_undirected.edges().collect::<Vec<_>>() {
        if g_undirected.has_edge(v, u) && v < u {
            g_undirected.add_edge_dups_ok(v, u);
            g_undirected.add_edge_dups_ok(u, v);
        } else {
            g_undirected.add_edge(v, u);
        }
    }
    //add reverse edges

    //let cycles = simple_cycles(&G);

    //let connected_components = strongly_connected_components(&G);

    //Cycles are\n{:?}\ncc's are\n{:?}\n"
    debug!(
        "P is\n{:?}\nGraph is\n{:?}\n",
        P,
        g_undirected.edges().collect::<Vec<_>>(),
        // cycles,
        // connected_components
    );

    let mut edge_values: Vec<(usize, usize, i64)> = Vec::new();

    let mut bfs_visited = BitSet::new();

    for f in 1..=F {
        if bfs_visited.contains(f) {
            continue;
        }

        let cc = g_undirected.bfs(f).collect::<Vec<_>>();
        bfs_visited.extend(cc.clone());

        let mut subG = g_undirected.subgraph(&cc);
        //for (u, v) in subG.edges().collect::<Vec<_>>() {}
        debug!("CC {:?}\nsubG {:?}", cc, subG.edges().collect::<Vec<_>>());

        //spanning tree
        let mut ST = DiGraph::new();

        let mut visited = BitVec::from_elem(subG.max_v() + 1, false);

        let mut discovery_order = Vec::new();

        dfs(&mut discovery_order, &mut ST, &subG, cc[0]);

        for st_edge in ST.edges() {
            subG.remove_undirected_edge(st_edge.0, st_edge.1);
        }

        debug!(
            "For sub graph {:?} spanning tree is {:?}",
            subG.edges().collect::<Vec<_>>(),
            ST.edges().collect::<Vec<_>>()
        );

        visited.clear();
        debug!("Discovery order is {:?} ", discovery_order);

        //Direct all edges in root-to-leaf direction
        for subG_edge in subG.edges().collect::<Vec<_>>() {
            let pos1 = discovery_order
                .iter()
                .position(|&d| d == subG_edge.0)
                .unwrap();
            let pos2 = discovery_order
                .iter()
                .position(|&d| d == subG_edge.1)
                .unwrap();

            if pos1 > pos2 {
                subG.remove_edge(subG_edge.0, subG_edge.1);
            }
        }

        debug!(
            "For sub graph directed root->leaf {:?}",
            subG.edges().collect::<Vec<_>>()
        );

        //root is automatically balanced
        discovery_order.reverse();
        discovery_order.pop();

        for current_node in discovery_order {
            visited.set(current_node, true);

            let tree_children: Vec<_> = ST.edges_from(current_node).collect();
            let tree_parents: Vec<_> = ST.edges_to(current_node).collect();
            assert_eq!(tree_parents.len(), 1);
            let tree_parent = tree_parents[0];

            let non_tree_edges_ancestor: Vec<_> = subG
                .edges()
                .filter(|&edge| edge.1 == current_node)
                .map(|edge| edge.0)
                .collect();
            let non_tree_edges_descendent: Vec<_> =
                subG.edges_from(current_node).filter(|&n| visited[n]).collect();

            debug!("Looking at tree children {:?} tree parent {}\nnon tree dges {:?}\nfor current node {}", tree_children, tree_parent,
                       non_tree_edges_ancestor,
                       current_node);

            //this->parent edge is not in the tree, was initially assigned val. of 1
            let mut balanced_value: i64 = 0;
            //ancestor nodes
            for v in non_tree_edges_ancestor {
                /*Direct all edges in root-to-leaf direction
                 (we reverse or split edges after solving, as explained above).
                  We assign edges not in the tree a value of 1,
                that is, they send positive news from nodes to descendants. */
                //assert!(!edge_values.contains(&(dis_node,v)));
                //assert!(edge_values.insert((dis_node,v),-1)==None);
                edge_values.push((v, current_node, 1));
                balanced_value += 1;
            }

            for v in non_tree_edges_descendent {
                //these have already been assigned
                balanced_value -= 1;
            }

            for t in tree_children {
                assert!(visited[t]);

                balanced_value -= edge_values
                    .iter()
                    .filter(|&ev| ev.0 == current_node && ev.1 == t && ev.2 != 1)
                    .map(|ev| ev.2)
                    .sum::<i64>();
                //.get( &(dis_node, t) ).unwrap();
            }

            edge_values.push((tree_parent, current_node, -balanced_value));
            //assert!(None==edge_values.insert((tree_parent, dis_node), - balanced_value));
            //*edge_values.entry( (tree_parent, dis_node)).or_insert(0)  -= balanced_value;
        }
    }

    debug!("Edge values are {:?}", edge_values);

    if edge_values.iter().any(|ev| ev.2 == 0) {
        return format!("Case #{}: IMPOSSIBLE\n", case_no);
    }

    let mut ans: Vec<i64> = Vec::new();
    for fe in P {
        if let Some(pos) = edge_values.iter().position(|&e| e.0 == fe.0 && e.1 == fe.1) {
            ans.push(edge_values[pos].2);
            edge_values.remove(pos);
            continue;
        } else if let Some(pos) = edge_values.iter().position(|&e| e.0 == fe.1 && e.1 == fe.0) {
            ans.push(-edge_values[pos].2);
            edge_values.remove(pos);
            continue;
        } else {
            return format!("Case #{}: IMPOSSIBLE\n", case_no);
        }
    }

    //debug!("G {:?} {}", digits, G,);
    let mut check_sums = vec![0; F];

    for (p, a) in P.iter().zip(ans.iter()) {
        check_sums[p.0 - 1] -= *a;
        check_sums[p.1 - 1] += *a;
    }

    if check_sums.iter().any(|cs| *cs != 0) {
        println!("Check sum failed: {:?} case {}", check_sums, case_no);
    }

    format!(
        "Case #{}: {}\n",
        case_no,
        ans.iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    )
}

fn dfs(discovery_order: &mut Vec<usize>, ST: &mut DiGraph, subG: &DiGraph, u: usize)
{
    discovery_order.push(u);
    for v in subG.edges_from(u) {
        if !ST.has_vertex(v) {
            //root to leaf direction
            ST.add_edge(u, v);

            dfs(discovery_order, ST, subG, v);
        }
    }
}
