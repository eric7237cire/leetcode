/// A compact graph representation. Edges are numbered in order of insertion.
/// Each adjacency list consists of all edges pointing out from a given vertex.
pub struct Graph
{
    /// Maps a vertex id to the first edge in its adjacency list.  New edges are added to the front
    first: Vec<Option<usize>>,
    /// Maps an edge id to the next edge in the same adjacency list.
    /// An edge can only be in 1 adj. list since each edge only has one 'from'
    next: Vec<Option<usize>>,
    /// Maps an edge id to the vertex that it points to.
    pub(super) endp: Vec<usize>,
}

impl Graph
{
    /// Initializes a graph with vmax vertices and no edges. To reduce
    /// unnecessary allocations, emax_hint should be close to the number of
    /// edges that will be inserted.
    pub fn new(vmax: usize, emax_hint: usize) -> Self
    {
        Self {
            first: vec![None; vmax],
            next: Vec::with_capacity(emax_hint),
            endp: Vec::with_capacity(emax_hint),
        }
    }

    /// Returns the number of vertices.
    pub fn num_v(&self) -> usize
    {
        self.first.len()
    }

    /// Returns the number of edges, double-counting undirected edges.
    pub fn num_e(&self) -> usize
    {
        self.endp.len()
    }

    /// Adds a directed edge from u to v.
    pub fn add_edge(&mut self, u: usize, v: usize)
    {
        self.next.push(self.first[u]);
        self.first[u] = Some(self.num_e());
        self.endp.push(v);
    }

    /// An undirected edge is two directed edges. If edges are added only via
    /// this funcion, the reverse of any edge e can be found at e^1.
    pub fn add_undirected_edge(&mut self, u: usize, v: usize)
    {
        self.add_edge(u, v);
        self.add_edge(v, u);
    }

    /// Gets vertex u's adjacency list.
    pub fn adj_list(&self, u: usize) -> AdjListIterator
    {
        AdjListIterator {
            graph: self,
            next_e: self.first[u],
        }
    }
}

/// An iterator for convenient adjacency list traversal.
pub struct AdjListIterator<'a>
{
    graph: &'a Graph,
    next_e: Option<usize>,
}

impl<'a> Iterator for AdjListIterator<'a>
{
    type Item = (usize, usize);

    /// Produces an outgoing edge and vertex.
    fn next(&mut self) -> Option<Self::Item>
    {
        self.next_e.map(|e| {
            let v = self.graph.endp[e];
            self.next_e = self.graph.next[e];
            (e, v)
        })
    }
}


#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn test_graph()
    {
        let mut graph = Graph::new(4, 4);

        assert_eq!(graph.first, vec![None;4]);
        assert_eq!(graph.next, vec![Some(0usize);0]);
        assert_eq!(graph.endp, vec![0usize;0]);

        graph.add_edge(0, 1);

        assert_eq!(graph.first, vec![Some(0), None, None, None]);
        assert_eq!(graph.next, vec![None;1]);
        assert_eq!(graph.endp, vec![1]);

        graph.add_edge(0, 2);

        assert_eq!(graph.first, vec![Some(1), None, None, None]);
        assert_eq!(graph.next, vec![None, Some(0)]);
        assert_eq!(graph.endp, vec![1, 2]);

    }
}