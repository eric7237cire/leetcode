/*https://github.com/networkx/networkx/blob/2736e7649c8c8e7aa5bc8f3745043d2fa24aaf9f/networkx/algorithms/cycles.py
https://github.com/networkx/networkx/blob/2736e7649c8c8e7aa5bc8f3745043d2fa24aaf9f/networkx/algorithms/tests/test_cycles.py
*/
use std::collections::HashSet;
use std::collections::HashMap;
use crate::algo::graph::DirectedGraph;

fn simple_cycles(G: &DirectedGraph) -> Vec<Vec<usize>>
{
    /* """Find simple cycles (elementary circuits) of a directed graph.

    An simple cycle, or elementary circuit, is a closed path where no
    node appears twice, except that the first and last node are the same.
    Two elementary circuits are distinct if they are not cyclic permutations
    of each other.

    This is a nonrecursive, iterator/generator version of Johnson's
    algorithm [1]_.  There may be better algorithms for some cases [2]_ [3]_.

    Parameters
    ----------
    G : NetworkX DiGraph
       A directed graph

    Returns
    -------
    cycle_generator: generator
       A generator that produces elementary cycles of the graph.  Each cycle is
       a list of nodes with the first and last nodes being the same.

    Examples
    --------
    >>> G = nx.DiGraph([(0, 0), (0, 1), (0, 2), (1, 2), (2, 0), (2, 1), (2, 2)])
    >>> list(nx.simple_cycles(G))
    [[2], [2, 1], [2, 0], [2, 0, 1], [0]]

    Notes
    -----
    The implementation follows pp. 79-80 in [1]_.

    The time complexity is O((n+e)(c+1)) for n nodes, e edges and c
    elementary circuits.

    To filter the cycles so that they don't include certain nodes or edges,
    copy your graph and eliminate those nodes or edges before calling.
    >>> copyG = G.copy()
    >>> copyG.remove_nodes_from([1])
    >>> copyG.remove_edges_from([(0,1)])
    >>> list(nx.simple_cycles(copyG))
    [[2], [2, 0], [0]]

    References
    ----------
    .. [1] Finding all the elementary circuits of a directed graph.
       D. B. Johnson, SIAM Journal on Computing 4, no. 1, 77-84, 1975.
       http://dx.doi.org/10.1137/0204007

    .. [2] Enumerating the cycles of a digraph: a new preprocessing strategy.
       G. Loizou and P. Thanish, Information Sciences, v. 27, 163-182, 1982.

    .. [3] A search strategy for the elementary cycles of a directed graph.
       J.L. Szwarcfiter and P.E. Lauer, BIT NUMERICAL MATHEMATICS,
       v. 16, no. 2, 192-204, 1976.

    See Also
    --------
    cycle_basis
    """
        */
    fn _unblock(
        thisnode: usize,
        blocked: &mut HashSet<usize>,
        B: &mut HashMap<usize, HashSet<usize>>,
    )
    {
        let mut stack: HashSet<usize> = HashSet::new();
        stack.insert(thisnode);
        while !stack.is_empty() {
            let node = *stack.iter().next().unwrap();
            stack.remove(&node);
            if blocked.contains(&node) {
                blocked.remove(&node);
                //simulate python default dict
                if B.contains_key(&node) {
                    stack.extend(B[&node].iter());
                }
                B.insert(node, HashSet::new());
            }
        }
    }

    /*
    # Johnson's algorithm requires some ordering of the nodes.
    # We assign the arbitrary ordering given by the strongly connected comps
    # There is no need to track the ordering as each node removed as processed.
    */
    //subG = type (G)(G.edges_iter()); /*# save the actual graph so we can mutate it here
    // # We only take the edges because we do not want to
    //  # copy edge and node attributes here.*/
    let mut sccs = strongly_connected_components(G);

    let mut ans: Vec<Vec<usize>> = Vec::new();

    sccs.retain(|sc| sc.len() > 1);

    let mut subG_edges = G.edges().collect::<Vec<_>>();

    for self_edge in subG_edges.iter().filter(|(u, v)| u == v) {
        ans.push(vec![self_edge.0]);
    }

    subG_edges.retain(|uv| uv.0 != uv.1);

    let mut subG: DirectedGraph = subG_edges.iter().collect();

    while !sccs.is_empty() {
        let mut scc = sccs.pop().unwrap();
        let sccG = subG.subgraph(&scc[..]);
        //already handled self loops
        if scc.len() <= 1 {
            continue;
        }

        //# order of scc determines ordering of nodes
        let startnode = scc.pop().unwrap();
        //# Processing node runs "circuit" routine from recursive version
        let mut path = vec![startnode];
        let mut blocked = HashSet::new(); //# vertex: blocked from search?
        let mut closed: HashSet<usize> = HashSet::new(); //# nodes involved in a cycle
        blocked.insert(startnode);
        let mut B: HashMap<usize, HashSet<usize>> = HashMap::new(); //# graph portions that yield no elementary circuit
        let mut stack: Vec<(usize, Vec<usize>)> =
            vec![(startnode, sccG.adj_list(startnode).collect())]; //# subG gives component nbrs
        while !stack.is_empty() {
            let (thisnode, nbrs) = stack.last_mut().unwrap();
            let thisnode = *thisnode;

            if !nbrs.is_empty() {
                let nextnode = nbrs.pop().unwrap();
                if nextnode == startnode {
                    ans.push(path.clone());
                    closed.extend(path.iter());
                //#                        print "Found a cycle",path,closed
                } else if !blocked.contains(&nextnode) {
                    path.push(nextnode);
                    stack.push((nextnode, sccG.adj_list(nextnode).collect()));
                    closed.remove(&nextnode);
                    blocked.insert(nextnode);
                    continue;
                }
            } //# done with nextnode... look for more neighbors
            if nbrs.is_empty() {
                //# no more nbrs
                if closed.contains(&thisnode) {
                    _unblock(thisnode, &mut blocked, &mut B);
                } else {
                    for nbr in sccG.adj_list(thisnode) {
                        if !B.entry(nbr).or_insert(HashSet::new()).contains(&thisnode) {
                            B.get_mut(&nbr).unwrap().insert(thisnode);
                        }
                    }
                }

                stack.pop();
                //#                assert path[-1]==thisnode
                path.pop();
            }
        }
        //# done processing this node

        let H = subG.subgraph(&scc[..]); //# make smaller to avoid work in SCC routine
        sccs.extend(strongly_connected_components(&H));
    }

    ans
}

#[cfg(test)]
mod test
{
    use super::*;
    //use std::collections::HashSet;

    fn double_sort(v: &mut Vec<Vec<usize>>)
    {
        for vv in v.iter_mut() {
            vv.sort();
        }
        v.sort();
    }

    //https://github.com/networkx/networkx/blob/bf1c7cc9b144767523e5abcf84f949d4223848a0/networkx/algorithms/components/tests/test_strongly_connected.py

    fn is_cyclic_permutation(a: &Vec<usize>, b: &Vec<usize>) -> bool
    {
        let n = a.len();
        if b.len() != n {
            return false;
        }
        let l: Vec<usize> = a.iter().chain(a.iter()).map(|e| *e).collect();

        for i in 0..n {
            if &l[i..i + n] == &b[..] {
                return true;
            }
        }
        return false;
    }

    #[test]
    fn test_simple_cycles()
    {
        let edges: Vec<(usize, usize)> =
            vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 0), (2, 1), (2, 2)];
        let G: DirectedGraph = edges.iter().collect();
        let mut cc = simple_cycles(&G);
        cc.sort();
        let ca = vec![vec![0], vec![0, 1, 2], vec![0, 2], vec![1, 2], vec![2]];

        println!("CC {:?}  correct: {:?}", cc, ca);
        assert_eq!(cc.len(), ca.len());
        for c in cc {
            //assert_true(any(self.is_cyclic_permutation(c, rc);
        }
        //for rc in ca))
    }
}
