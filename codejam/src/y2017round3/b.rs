use super::super::util::input::*;
use crate::algo::graph::cycles::simple_cycles;
use crate::algo::graph::DiGraph;
use crate::algo::graph::scc::strongly_connected_components;
//use std::collections::HashMap;
use std::io::Write;
use std::time::Instant;
use bit_vec::BitVec;
use std::collections::HashMap;

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

        print!("{}", solve(case, &G, &P));
    }

    let duration = now.elapsed();
    let secs = f64::from(duration.as_secs() as u32) + f64::from(duration.subsec_nanos()) / 1e9f64;
    let _ = writeln!(
        ::std::io::stderr(),
        "\nElapsed time {:.2} second(s)\n",
        secs
    );
}

fn solve(case_no: u32, G: &DiGraph, P: &Vec<(usize, usize)>) -> String
{
    debug!("Solving case {}", case_no);

    let mut G = G.clone();
    //add reverse edges


    let cycles = simple_cycles(&G);

    let connected_components = strongly_connected_components(&G);

    debug!("P is\n{:?}\nGraph is\n{:?}\nCycles are\n{:?}\ncc's are\n{:?}\n",
           P,
           G.edges().collect::<Vec<_>>(), cycles, connected_components);

    let mut edge_values:Vec< (usize,usize, i64)> = Vec::new();

    for cc in connected_components.iter() {
        let mut subG = G.subgraph(&cc);
        for (u, v) in subG.edges().collect::<Vec<_>>() {
            if subG.has_edge(v,u) && v < u {
                subG.add_edge_dups_ok(v, u);
                subG.add_edge_dups_ok(u, v);
            }
            else {subG.add_edge(v, u);}
        }
        debug!("CC {:?}\nsubG {:?}", cc, subG.edges().collect::<Vec<_>>());

        //spanning tree
        let mut ST = DiGraph::new();

        let mut visited = BitVec::from_elem(subG.max_v()+1, false);


        let mut discovery_order = Vec::new();

        dfs(&mut discovery_order, &mut ST, &subG, cc[0]);

        for st_edge in ST.edges() {
            subG.remove_undirected_edge(st_edge.0, st_edge.1);
        }

        debug!("For {:?} spanning tree is {:?}",
            subG.edges().collect::<Vec<_>>(), ST.edges().collect::<Vec<_>>());


        visited.clear();
        debug!("Discovery order is {:?} ", discovery_order);
        //root is automatically balanced
        discovery_order.reverse();
        discovery_order.pop();

        for dis_node in discovery_order {


            visited.set(dis_node, true);

            let tree_children:Vec<_> = ST.adj_list(dis_node).collect();
            let tree_parents:Vec<_> = ST.edges().filter(|e| e.1 == dis_node).collect();
            assert_eq!(tree_parents.len(), 1);
            let tree_parent = tree_parents[0].0;

            let non_tree_edges:Vec<_> = subG.adj_list(dis_node).collect();

            debug!("Looking at tree children {:?} tree parent {}\nnon tree dges {:?}\nfor current node {}", tree_children, tree_parent,
                   non_tree_edges,
                   dis_node);

            //this->parent edge is not in the tree, was initially assigned val. of 1
            let mut balanced_value:i64 = 0;
            //ancestor nodes
            for v in non_tree_edges {
                /*Direct all edges in root-to-leaf direction
                 (we reverse or split edges after solving, as explained above).
                  We assign edges not in the tree a value of 1,
                that is, they send positive news from nodes to descendants. */
                //assert!(!edge_values.contains(&(dis_node,v)));
                //assert!(edge_values.insert((dis_node,v),-1)==None);
                edge_values.push((v, dis_node, 1));
                balanced_value += 1;
            }



            for t in tree_children {
                assert!(visited[t]);

                balanced_value -= edge_values.iter().filter( |&ev|
                                                                 ev.0 == dis_node && ev.1 == t)
                    .map( |ev| ev.2)
                    .sum::<i64>();
                                                                 //.get( &(dis_node, t) ).unwrap();

            }

            edge_values.push( (tree_parent, dis_node, -balanced_value));
            //assert!(None==edge_values.insert((tree_parent, dis_node), - balanced_value));
            //*edge_values.entry( (tree_parent, dis_node)).or_insert(0)  -= balanced_value;


        }

        debug!("Edge values are {:?}", edge_values);
    }


   /* let ans = P.iter().map( |&fe| {
        let mut sum = 0;
        if let Some(pos) = news_values.get(&fe) {
            sum += pos;
            news_values.remove(&fe);
        }
        let other_edge = (fe.1, fe.0);
        if !P.contains(&other_edge) {
            if let Some(neg) = news_values.get(&other_edge) {
                sum -= neg;
            }
            news_values.remove(&other_edge);
        }
        sum
    }).collect::<Vec<_>>();



    if ans.contains(&"0".to_string()) {
        return format!("Case #{}: IMPOSSIBLE\n", case_no);
    }

    //debug!("G {:?} {}", digits, G,);

    if news_values.is_empty() {
        format!("Case #{}: {}\n", case_no, ans.join(" "))
    } else {
    */
        format!("Case #{}: IMPOSSIBLE\n", case_no)
}


fn dfs(discovery_order: &mut Vec<usize>, ST: &mut DiGraph, subG: &DiGraph, u: usize)
{
    discovery_order.push(u);
    for v in subG.adj_list(u) {
        if !ST.has_vertex(v) {
            //root to leaf direction
            ST.add_edge(u, v);

            dfs(discovery_order, ST, subG, v);
        }

    }
}