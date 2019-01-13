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
        if scc_found[source] {
            continue;
        }
        let mut queue = vec![source];
        while !queue.is_empty() {
            let v:usize = *queue.last().unwrap();
            println!("Processing v={} on queue", v);
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

    return_scc
}

/*
def simple_cycles(G):
    """Find simple cycles (elementary circuits) of a directed graph.

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
    def _unblock(thisnode,blocked,B):
        stack=set([thisnode])
        while stack:
            node=stack.pop()
            if node in blocked:
                blocked.remove(node)
                stack.update(B[node])
                B[node].clear()

    # Johnson's algorithm requires some ordering of the nodes.
    # We assign the arbitrary ordering given by the strongly connected comps
    # There is no need to track the ordering as each node removed as processed.
    subG = type(G)(G.edges_iter()) # save the actual graph so we can mutate it here
                              # We only take the edges because we do not want to
                              # copy edge and node attributes here.
    sccs = list(nx.strongly_connected_components(subG))
    while sccs:
        scc=sccs.pop()
        # order of scc determines ordering of nodes
        startnode = scc.pop()
        # Processing node runs "circuit" routine from recursive version
        path=[startnode]
        blocked = set() # vertex: blocked from search?
        closed = set() # nodes involved in a cycle
        blocked.add(startnode)
        B=defaultdict(set) # graph portions that yield no elementary circuit
        stack=[ (startnode,list(subG[startnode])) ]  # subG gives component nbrs
        while stack:
            thisnode,nbrs = stack[-1]
            if nbrs:
                nextnode = nbrs.pop()
#                    print thisnode,nbrs,":",nextnode,blocked,B,path,stack,startnode
#                    f=raw_input("pause")
                if nextnode == startnode:
                    yield path[:]
                    closed.update(path)
#                        print "Found a cycle",path,closed
                elif nextnode not in blocked:
                    path.append(nextnode)
                    stack.append( (nextnode,list(subG[nextnode])) )
                    closed.discard(nextnode)
                    blocked.add(nextnode)
                    continue
            # done with nextnode... look for more neighbors
            if not nbrs:  # no more nbrs
                if thisnode in closed:
                    _unblock(thisnode,blocked,B)
                else:
                    for nbr in subG[thisnode]:
                        if thisnode not in B[nbr]:
                            B[nbr].add(thisnode)
                stack.pop()
#                assert path[-1]==thisnode
                path.pop()
        # done processing this node
        subG.remove_node(startnode)
        H=subG.subgraph(scc)  # make smaller to avoid work in SCC routine
        sccs.extend(list(nx.strongly_connected_components(H)))
*/

#[cfg(test)]
mod test
{
    use super::*;
    use std::collections::HashSet;

    fn double_sort(v: &mut Vec<Vec<usize>>)
    {
        for vv in v.iter_mut() {
            vv.sort();
        }
        v.sort();
    }

    #[test]
    fn test_scc()
    {
        use std::iter::FromIterator;
        let pairs = vec![
(1, 2), (2, 3), (2, 8), (3, 4), (3, 7), (4, 5),
                          (5, 3), (5, 6), (7, 4), (7, 6), (8, 1), (8, 7)
        ];

        let sccs:Vec<Vec<usize>> = vec![
        vec![3, 4, 5, 7], vec![1, 2, 8], vec![6] ];

        let mut graph = Graph::new(8, 6);

        for p in pairs {
            graph.add_edge(p.0-1, p.1-1);
        }

        let mut ans = strongly_connected_components(&graph);

        double_sort(&mut ans);
        let mut check_ans= sccs.iter()
            .map( |a| a.iter().map( |b| b-1).collect::<Vec<usize>>()
            ).collect::<Vec<Vec<usize>>>();
        double_sort(&mut check_ans);

        println!("{:?} correct: {:?}", ans, check_ans);

        assert_eq!(ans.len(), check_ans.len());
        assert_eq!(ans, check_ans);

    }
}
