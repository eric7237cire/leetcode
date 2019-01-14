//https://stackoverflow.com/questions/46511682/non-recursive-version-of-tarjans-algorithm

//https://www.geeksforgeeks.org/iterative-depth-first-traversal/

//! Graph connectivity structures.
use super::DirectedGraph;
use bit_vec::BitVec;
use std::cmp::min;
use std::collections::HashMap;
use std::collections::HashSet;

//https://networkx.github.io/documentation/networkx-1.9.1/_modules/networkx/algorithms/components/strongly_connected.html#strongly_connected_components
fn strongly_connected_components(G: &DirectedGraph) -> Vec<Vec<usize>>
{
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
    let mut scc_queue: Vec<usize> = vec![];
    let mut i = 0; //     # Preorder counter
    let mut return_scc = Vec::new();

    for source in 0..G.num_v() {
        //println!("Source is {}", source);
        if scc_found[source] {
            continue;
        }
        let mut queue = vec![source];
        while !queue.is_empty() {
            let v: usize = *queue.last().unwrap();
            //println!("Processing v={} on queue", v);
            if preorder[v] == None {
                i = i + 1;
                preorder[v] = Some(i);
            }
            let mut done = true;

            for w in G.adj_list(v) {
                if preorder[w] == None {
                    queue.push(w);
                    done = false;
                    break;
                }
            }
            if done {
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
                    while !scc_queue.is_empty()
                        && preorder[*scc_queue.last().unwrap()] > preorder[v]
                    {
                        let k = scc_queue.pop().unwrap();
                        scc_found.set(k, true);
                        scc.push(k);
                    }
                    return_scc.push(scc);
                    continue;
                } else {
                    scc_queue.push(v);
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
        //use std::iter::FromIterator;
        let pairs: Vec<(usize, usize)> = vec![
            (1, 2),
            (2, 3),
            (2, 8),
            (3, 4),
            (3, 7),
            (4, 5),
            (5, 3),
            (5, 6),
            (7, 4),
            (7, 6),
            (8, 1),
            (8, 7),
        ];

        let sccs: Vec<Vec<usize>> = vec![vec![3, 4, 5, 7], vec![1, 2, 8], vec![6]];

        let mut graph = DirectedGraph::new();

        for p in pairs {
            graph.add_edge(p.0 - 1, p.1 - 1);
        }

        let mut ans = strongly_connected_components(&graph);

        double_sort(&mut ans);
        let mut check_ans = sccs
            .iter()
            .map(|a| a.iter().map(|b| b - 1).collect::<Vec<usize>>())
            .collect::<Vec<Vec<usize>>>();
        double_sort(&mut check_ans);

        println!("{:?} correct: {:?}", ans, check_ans);

        assert_eq!(ans.len(), check_ans.len());
        assert_eq!(ans, check_ans);
    }
}
