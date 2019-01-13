//https://stackoverflow.com/questions/46511682/non-recursive-version-of-tarjans-algorithm

//https://www.geeksforgeeks.org/iterative-depth-first-traversal/

//! Graph connectivity structures.
use super::Graph;
use bit_vec::BitVec;
use std::cmp::min;

//https://networkx.github.io/documentation/networkx-1.9.1/_modules/networkx/algorithms/components/strongly_connected.html#strongly_connected_components
fn strongly_connected_components(G: &Graph) -> Vec<Vec<usize>>{
    /*Generate nodes in strongly connected components of graph.

    Parameters
    ----------
    G : NetworkX Graph
       An directed graph.

    Returns
    -------
    comp : generator of lists
       A list of nodes for each strongly connected component of G.

    Raises
    ------
    NetworkXNotImplemented: If G is undirected.

    See Also
    --------
    connected_components, weakly_connected_components

    Notes
    -----
    Uses Tarjan's algorithm with Nuutila's modifications.
    Nonrecursive version of algorithm.

    References
    ----------
    .. [1] Depth-first search and linear graph algorithms, R. Tarjan
       SIAM Journal of Computing 1(2):146-160, (1972).

    .. [2] On finding the strongly connected components in a directed graph.
       E. Nuutila and E. Soisalon-Soinen
       Information Processing Letters 49(1): 9-14, (1994)..
    """
    */
    let mut preorder: Vec<Option<usize>> = vec![None; G.num_v()];
    let mut lowlink = vec![0; G.num_v()];
    let mut scc_found = BitVec::from_elem(G.num_v(), false);
    let mut scc_queue:Vec<usize> = vec![];
    let mut i = 0; //     # Preorder counter
    let mut return_scc = Vec::new();

    for source in 0..G.num_v() {
        println!("Source is {}", source);
        if !scc_found[source] {
            let mut queue = vec![source];
            while !queue.is_empty() {
                let v:usize = *queue.last().unwrap();
                println!("Processing v={} on queue", v);
                if preorder[v] == None {
                    i = i + 1;
                    preorder[v] = Some(i);
                }
                let mut done = 1;

                for w in G.adj_list(v) {
                    if preorder[w] == None {
                        queue.push(w);
                        done = 0;
                        break;
                    }
                }
                if done == 1 {
                    lowlink[v] = preorder[v].unwrap();
                    for w in G.adj_list(v) {
                        if !scc_found[w] {
                            if preorder[w] > preorder[v] {
                                lowlink[v] = min(lowlink[v], lowlink[w]);
                            } else {
                                lowlink[v] = min(lowlink[v], preorder[w].unwrap());
                            }
                        }
                    }
                    queue.pop();
                    if lowlink[v] == preorder[v].unwrap() {
                        scc_found.set(v, true);
                        let mut scc = vec![v];
                        while !scc_queue.is_empty() && preorder[*scc_queue.last().unwrap()] > preorder[v] {
                            let k = scc_queue.pop().unwrap();
                            scc_found.set(k,true);
                            scc.push(k);
                        }
                        return_scc.push( scc);
                        continue;
                    } else {
                        scc_queue.push(v);
                    }
                }
            }
        }
    }

    return_scc
}


#[cfg(test)]
mod test
{
    use super::*;



    #[test]
    fn test_scc()
    {
        let pairs = vec![
(3, 4),
(1, 3),
(7, 4),
(2, 3),
(4, 5),
(4, 6),
(1, 2),
(2, 4),
(1, 4),
(6, 7),
        ];

        let mut graph = Graph::new(pairs.len(), 6);
        let mut graph2 = super::super::super::super::algo_ebtech::graph::Graph::new(pairs.len(), 6);

        for p in pairs {
            graph.add_undirected_edge(p.0, p.1);
            graph2.add_undirected_edge(p.0, p.1);
        }

        let cg = super::super::super::super::algo_ebtech::graph::connectivity::ConnectivityGraph::new(&graph2, true);
        assert_eq!(cg.num_cc, 4);

        let ccs = strongly_connected_components(&graph);

        println!("{:?}", ccs);

        assert_eq!(ccs.len(), 4);

    }
}
