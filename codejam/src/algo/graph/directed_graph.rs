use bit_vec::BitVec;
use std::cmp::max;
use std::iter::FromIterator;

/// A compact graph representation. Edges are numbered in order of insertion.
/// Each adjacency list consists of all edges pointing out from a given vertex.
pub struct DirectedGraph
{
    pub adj_list: Vec<Vec<usize>>,
    exists: BitVec
}

impl DirectedGraph
{
    /// Initializes a graph with vmax vertices and no edges. To reduce
    /// unnecessary allocations, emax_hint should be close to the number of
    /// edges that will be inserted.
    pub fn new() -> Self
    {
        Self {
            adj_list: Vec::new(),
            exists: BitVec::new()
        }
    }

    /// Returns the number of vertices.
    pub fn num_v(&self) -> usize
    {
        self.adj_list.len()
    }


    /// Adds a directed edge from u to v.
    pub fn add_edge(&mut self, u: usize, v: usize)
    {
        for _ in self.adj_list.len() ..= max(u,v) {
            self.exists.push(false);
            self.adj_list.push(Vec::new());
        }
        self.exists.set(u, true);
        self.exists.set(v,true);
        self.adj_list[u].push(v);
    }




    pub fn edges<'a>(&'a self) -> impl Iterator<Item = (usize, usize)> + 'a
    {
        (0..self.adj_list.len())
            .map(move |u| self.adj_list[u].iter().map(move |v| (u, *v)))
            .flatten()
    }
}

impl FromIterator<(usize,usize)> for DirectedGraph {
    fn from_iter<I: IntoIterator<Item=(usize,usize)>>(iter: I) -> Self {
        let mut c = DirectedGraph::new();

        for i in iter {
            c.add_edge(i.0,i.1);
        }

        c
    }
}
impl <'a> FromIterator<&'a( usize,usize)> for DirectedGraph {
    fn from_iter<I: IntoIterator<Item=&'a (usize,usize)>>(iter: I) -> Self {
        let mut c = DirectedGraph::new();

        for i in iter {
            c.add_edge(i.0,i.1);
        }

        c
    }
}
impl FromIterator<(i32,i32)> for DirectedGraph {
    fn from_iter<I: IntoIterator<Item=(i32,i32)>>(iter: I) -> Self {
        let mut c = DirectedGraph::new();

        for i in iter {
            c.add_edge(i.0 as usize,i.1 as usize);
        }

        c
    }
}


#[cfg(test)]
mod test_directed_graph
{
    use super::*;



    //cargo test test_edge_iterator -- --nocapture
    #[test]
    fn test_edge_iterator()
    {
        let mut graph = DirectedGraph::new();
        graph.add_edge(2, 2);
        graph.add_edge(2, 3);
        graph.add_edge(1, 0);
        graph.add_edge(3, 0);
        graph.add_edge(3, 2);

        assert_eq!(
            graph.edges().collect::<Vec<_>>(),
            vec![(1, 0), (2, 2), (2, 3), (3, 0), (3, 2)]
        );
    }

    #[test]
    fn test_collect()
    {
        let pairs:Vec<(usize,usize)> = vec![
(1, 2), (2, 3), (2, 8), (3, 4), (3, 7), (4, 5),
        ];
        let graph: DirectedGraph = pairs.iter().collect();

        assert_eq!(
            graph.edges().collect::<Vec<_>>(),
            vec![(1, 2), (2, 3), (2, 8), (3, 4), (3, 7), (4, 5),]
        );

    }

}
