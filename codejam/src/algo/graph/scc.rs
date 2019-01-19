//https://stackoverflow.com/questions/46511682/non-recursive-version-of-tarjans-algorithm

//https://www.geeksforgeeks.org/iterative-depth-first-traversal/

//! Graph connectivity structures.
use super::DiGraph;
use bit_vec::BitVec;
use std::cmp::min;
//use std::collections::HashMap;
//use std::collections::HashSet;

//https://networkx.github.io/documentation/networkx-1.9.1/_modules/networkx/algorithms/components/strongly_connected.html#strongly_connected_components
//TODO have this return an iterator
pub fn strongly_connected_components(G: &DiGraph) -> Vec<Vec<usize>>
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
    let mut preorder: Vec<Option<usize>> = vec![None; G.max_v()];
    let mut lowlink = vec![0; G.max_v()];
    let mut scc_found = BitVec::from_elem(G.max_v(), false);
    let mut scc_queue: Vec<usize> = vec![];
    let mut next_preorder_value = 0; //     # Preorder counter
    let mut return_scc = Vec::new();

    for source in 0..G.max_v() {
        if !G.has_vertex(source) {
            continue;
        }
        //println!("Source is {}", source);
        if scc_found[source] {
            continue;
        }
        let mut queue = vec![source];
        while let Some(v) = queue.last() {
            let v = *v;

            //println!("Processing v={} on queue", v);
            if preorder[v] == None {
                next_preorder_value += 1;
                preorder[v] = Some(next_preorder_value);
            }

            //Used to delay the rest until the queue gets back to this value
            let mut done = true;

            for w in G.edges_from(v) {
                if preorder[w] == None {
                    queue.push(w);
                    done = false;
                    break;
                }
            }
            if !done {
                continue;
            }
            lowlink[v] = preorder[v].unwrap();
            for w in G.edges_from(v).filter(|&w| !scc_found[w]) {
                lowlink[v] = min(
                    lowlink[v],
                    if preorder[w] > preorder[v] {
                        lowlink[w]
                    } else {
                        preorder[w].unwrap()
                    },
                );
            }
            queue.pop();
            if lowlink[v] == preorder[v].unwrap() {
                scc_found.set(v, true);
                let mut scc = vec![v];
                while !scc_queue.is_empty() && preorder[*scc_queue.last().unwrap()] > preorder[v] {
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

    return_scc
}

#[cfg(test)]
mod test_scc
{
    use super::*;

    //https://github.com/networkx/networkx/blob/master/networkx/algorithms/components/tests/test_strongly_connected.py

    fn double_sort(v: &mut Vec<Vec<usize>>)
    {
        for vv in v.iter_mut() {
            vv.sort();
        }
        v.sort();
    }

    fn verify_scc(pairs: &[(usize, usize)], sccs: &[Vec<usize>])
    {
        verify_scc_extra(pairs, sccs, Vec::new());
    }
    fn verify_scc_extra(pairs: &[(usize, usize)], sccs: &[Vec<usize>], extra_vertex: Vec<usize>)
    {
        let mut graph = DiGraph::new();

        for p in pairs {
            graph.add_edge(p.0, p.1);
        }
        for v in extra_vertex {
            graph.add_vertex(v);
        }

        let mut ans = strongly_connected_components(&graph);

        double_sort(&mut ans);
        let mut check_ans = sccs.to_vec();
        double_sort(&mut check_ans);

        println!("{:?} correct: {:?}", ans, check_ans);

        assert_eq!(ans.len(), check_ans.len());
        assert_eq!(ans, check_ans);
    }

    #[test]
    fn test_scc_1()
    {
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

        verify_scc(&pairs, &sccs);
    }

    #[test]
    fn test_scc_2()
    {
        let pairs: Vec<(usize, usize)> = vec![(1, 2), (1, 3), (1, 4), (4, 2), (3, 4), (2, 3)];

        let sccs: Vec<Vec<usize>> = vec![vec![2, 3, 4], vec![1]];

        verify_scc(&pairs, &sccs);
    }

    #[test]
    fn test_scc_3()
    {
        //use std::iter::FromIterator;
        let pairs: Vec<(usize, usize)> = vec![(1, 2), (2, 3), (3, 2), (2, 1)];

        let sccs: Vec<Vec<usize>> = vec![vec![1, 2, 3]];

        verify_scc(&pairs, &sccs);
    }

    //Eppstein's tests
    #[test]
    fn test_scc_4()
    {
        //use std::iter::FromIterator;
        let pairs: Vec<(usize, usize)> = vec![
            (0, 1),
            (1, 2),
            (1, 3),
            (2, 4),
            (2, 5),
            (3, 4),
            (3, 5),
            (4, 6),
        ];

        let sccs: Vec<Vec<usize>> = vec![
            vec![0],
            vec![1],
            vec![2],
            vec![3],
            vec![4],
            vec![5],
            vec![6],
        ];

        verify_scc(&pairs, &sccs);
    }

    #[test]
    fn test_scc_5()
    {
        //use std::iter::FromIterator;
        let pairs: Vec<(usize, usize)> = vec![
            (0, 1),
            (1, 2),
            (1, 3),
            (1, 4),
            (2, 0),
            (2, 3),
            (3, 4),
            (4, 3),
        ];

        let sccs: Vec<Vec<usize>> = vec![vec![0, 1, 2], vec![3, 4]];

        verify_scc(&pairs, &sccs);
    }

    #[test]
    fn test_empty_scc()
    {
        //use std::iter::FromIterator;
        let pairs: Vec<(usize, usize)> = vec![];

        let sccs: Vec<Vec<usize>> = vec![];

        verify_scc(&pairs, &sccs);
    }

    #[test]
    fn test_scc_6()
    {
        //use std::iter::FromIterator;
        let pairs: Vec<(usize, usize)> = vec![(1, 2), (1, 3), (2, 3), (2, 1), (3, 1), (3, 2)];

        let sccs: Vec<Vec<usize>> = vec![vec![1, 2, 3]];

        verify_scc(&pairs, &sccs);
    }
}
