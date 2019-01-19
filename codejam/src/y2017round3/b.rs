use crate::algo::graph::DiGraph;
use crate::util::codejam::run_cases;
use bit_set::BitSet;
use std::io::Write;

/*
DFS
Spanning tree
Bridges
Cycles
Connected components
Double edges
Undirected graph
BFS
*/
pub fn solve_all_cases()
{
    run_cases(
        &["B-small-practice", "B-large-practice"],
        "y2017round3",
        |reader, buffer| {
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

                write!(buffer, "{}", solve(case, &G, &P, F)).unwrap();
            }
        },
    );
}

fn solve(case_no: u32, G: &DiGraph, P: &[(usize, usize)], F: usize) -> String
{
    debug!("\n\n\nSolving case {}", case_no);

    //Create an undirected graph with duplicates when u->v and v-> already exist in P
    let mut g_undirected = G.clone();
    for (u, v) in g_undirected.edges().collect::<Vec<_>>() {
        if g_undirected.has_edge(v, u) && v < u {
            g_undirected.add_edge_dups_ok(v, u);
            g_undirected.add_edge_dups_ok(u, v);
        } else {
            g_undirected.add_edge(v, u);
        }
    }

    debug!(
        "P is\n{:?}\nUndirected Graph is\n{:?}\n",
        P,
        g_undirected.edges().collect::<Vec<_>>(),
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
            let tree_children: Vec<_> = ST.edges_from(current_node).collect();
            let tree_parents: Vec<_> = ST.edges_to(current_node).collect();
            assert_eq!(tree_parents.len(), 1);
            let tree_parent = tree_parents[0];

            let mut balanced_value: i64 = 0;

            //These are edges not in the spanning tree that we assign 1 to
            for v in subG.edges_to(current_node) {
                //non_tree_edges_ancestor {
                /*Direct all edges in root-to-leaf direction
                 (we reverse or split edges after solving, as explained above).
                  We assign edges not in the tree a value of 1,
                that is, they send positive news from nodes to descendants. */

                edge_values.push((v, current_node, 1));
                balanced_value += 1;
            }

            //These are previously seen edges not in the spanning tree that we need to account for
            for _ in subG.edges_from(current_node) {
                //these have already been assigned
                balanced_value -= 1;
            }

            for t in tree_children {
                balanced_value -= edge_values
                    .iter()
                    //only count edges in the spanning tree which won't have a positive value
                    .filter(|&ev| ev.0 == current_node && ev.1 == t && ev.2 != 1)
                    .map(|ev| ev.2)
                    .sum::<i64>();
                //.get( &(dis_node, t) ).unwrap();
            }

            edge_values.push((tree_parent, current_node, -balanced_value));
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

    /*
    let mut check_sums = vec![0; F];

    for (p, a) in P.iter().zip(ans.iter()) {
        check_sums[p.0 - 1] -= *a;
        check_sums[p.1 - 1] += *a;
    }

    if check_sums.iter().any(|cs| *cs != 0) {
        println!("Check sum failed: {:?} case {}", check_sums, case_no);
    }*/

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
