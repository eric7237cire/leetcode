//use bit_vec::BitVec;
//use std::cmp::max;
use std::iter::FromIterator;
use bit_set::BitSet;

/// A compact graph representation. Edges are numbered in order of insertion.
/// Each adjacency list consists of all edges pointing out from a given vertex.
#[derive(Clone)]
pub struct DiGraph
{
    exists: BitSet,
    has_edge_from: Vec<BitSet>,
    has_edge_to: Vec<BitSet>
}

impl DiGraph
{
    /// Initializes a graph with vmax vertices and no edges. To reduce
    /// unnecessary allocations, emax_hint should be close to the number of
    /// edges that will be inserted.
    pub fn new() -> Self
    {
        Self {

            //Does vertex exist? index == vertex
            exists: BitSet::new(),

            //has_edge[u][v] is u->v an edge in the graph
            has_edge_from: Vec::new(),

            has_edge_to: Vec::new(),
        }
    }

    pub fn complete_graph(n: usize) -> Self
    {
        let mut g = Self::new();

        for u in 0..n {
            for v in 0..n {
                if u == v {
                    continue;
                }
                g.add_edge(u, v);
            }
        }

        g
    }

    /// Returns the number of vertices.
    pub fn max_v(&self) -> usize
    {
        self.has_edge_from.len()
    }

    pub fn has_vertex(&self, v: usize) -> bool
    {
        self.exists.contains(v)
    }

    pub fn has_edge(&self, u: usize, v: usize) -> bool
    {
        self.has_edge_from.len() > u && self.has_edge_from[u].contains(v)
    }
    pub fn add_vertex(&mut self, v: usize)
    {
        for _ in self.has_edge_from.len()..=v {
            self.has_edge_from.push(BitSet::new());
            self.has_edge_to.push(BitSet::new());
        }
        self.exists.insert(v);
    }

    /// Adds a directed edge from u to v.
    pub fn add_edge(&mut self, u: usize, v: usize)
    {
        //disallow duplicate edges
        if !self.has_edge(u, v) {
            self.add_vertex(u);
            self.add_vertex(v);

            self.has_edge_from[u].insert(v);
            self.has_edge_to[v].insert(u);
        }
    }

    pub fn add_undirected_edge(&mut self, u: usize, v: usize) {
        self.add_edge(u,v);
        self.add_edge(v,u);
    }

    pub fn remove_edge(&mut self, u: usize, v: usize)
    {
        self.has_edge_to[v].remove(u);
        self.has_edge_from[u].remove(v);
    }

    pub fn remove_undirected_edge(&mut self, u: usize, v: usize)
    {
        self.remove_edge(u,v);
        self.remove_edge(v,u);
    }

    pub fn subgraph(&self, nodes: &[usize]) -> DiGraph
    {
        let mut sg: Self = DiGraph::new();
        for n in nodes.iter() {
            sg.add_vertex(*n);
        }
        for uv in self.edges() {
            if sg.has_vertex(uv.0) && sg.has_vertex(uv.1) {
                sg.add_edge(uv.0, uv.1);
            }
        }

        sg
    }

    pub fn remove_node(&mut self, node: usize)
    {
        self.exists.remove(node);
        if node < self.has_edge_from.len() {
            self.has_edge_from[node].clear();
        }
        for to in self.has_edge_to.iter_mut() {
            to.remove(node);
        }
    }

    pub fn edges_from<'a>(&'a self, node: usize) -> impl Iterator<Item = usize> + 'a
    {
        self.has_edge_from[node].iter()
    }

    pub fn edges_to<'a>(&'a self, node: usize) -> impl Iterator<Item = usize> + 'a
    {
        self.has_edge_to[node].iter()
    }

    pub fn edges<'a>(&'a self) -> impl Iterator<Item = (usize, usize)> + 'a
    {
        (0..self.has_edge_from.len())
            .map(move |u| self.has_edge_from[u].iter().map(move |v| (u, v)))
            .flatten()
    }
}

impl FromIterator<(usize, usize)> for DiGraph
{
    fn from_iter<I: IntoIterator<Item = (usize, usize)>>(iter: I) -> Self
    {
        let mut c = DiGraph::new();

        for i in iter {
            c.add_edge(i.0, i.1);
        }

        c
    }
}
impl<'a> FromIterator<&'a (usize, usize)> for DiGraph
{
    fn from_iter<I: IntoIterator<Item = &'a (usize, usize)>>(iter: I) -> Self
    {
        let mut c = DiGraph::new();

        for i in iter {
            c.add_edge(i.0, i.1);
        }

        c
    }
}
impl FromIterator<(i32, i32)> for DiGraph
{
    fn from_iter<I: IntoIterator<Item = (i32, i32)>>(iter: I) -> Self
    {
        let mut c = DiGraph::new();

        for i in iter {
            c.add_edge(i.0 as usize, i.1 as usize);
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
        let mut graph = DiGraph::new();
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
        let pairs: Vec<(usize, usize)> = vec![(1, 2), (2, 3), (2, 8), (3, 4), (3, 7), (4, 5)];
        let graph: DiGraph = pairs.iter().collect();

        assert_eq!(
            graph.edges().collect::<Vec<_>>(),
            vec![(1, 2), (2, 3), (2, 8), (3, 4), (3, 7), (4, 5),]
        );
    }

    #[test]
    fn test_subgraph()
    {
        let pairs: Vec<(usize, usize)> = vec![(1, 2), (2, 3), (2, 8), (3, 4), (3, 7), (4, 5)];
        let mut graph: DiGraph = pairs.iter().collect();
        graph.add_vertex(14);

        let sg = graph.subgraph(&vec![1, 2, 8, 14]);
        assert_eq!(sg.edges().collect::<Vec<_>>(), vec![(1, 2), (2, 8)]);

        assert!(sg.has_vertex(14));
    }

}
