/// A compact graph representation. Edges are numbered in order of insertion.
/// Each adjacency list consists of all edges pointing out from a given vertex.
pub struct Graph
{
    /// Maps a vertex id to the first edge in its adjacency list.  New edges are added to the front
    vertex_to_first_edge: Vec<Option<usize>>,
    /// Maps an edge id to the next edge in the same adjacency list.
    /// An edge can only be in 1 adj. list since each edge only has one 'from'
    edge_to_next_edge: Vec<Option<usize>>,
    /// Maps an edge id to the vertex that it points to.
    pub endp: Vec<usize>,
}

impl Graph
{
    /// Initializes a graph with vmax vertices and no edges. To reduce
    /// unnecessary allocations, emax_hint should be close to the number of
    /// edges that will be inserted.
    pub fn new(vmax: usize, emax_hint: usize) -> Self
    {
        Self {
            vertex_to_first_edge: vec![None; vmax],
            edge_to_next_edge: Vec::with_capacity(emax_hint),
            endp: Vec::with_capacity(emax_hint),
        }
    }

    /// Returns the number of vertices.
    pub fn num_v(&self) -> usize
    {
        self.vertex_to_first_edge.len()
    }

    /// Returns the number of edges, double-counting undirected edges.
    pub fn num_e(&self) -> usize
    {
        self.endp.len()
    }

    /// Adds a directed edge from u to v.
    pub fn add_edge(&mut self, u: usize, v: usize)
    {
        self.edge_to_next_edge.push(self.vertex_to_first_edge[u]);
        self.vertex_to_first_edge[u] = Some(self.num_e());
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
    pub fn adj_list_with_edges(&self, u: usize) -> AdjListIteratorWithEdges
    {
        AdjListIteratorWithEdges {
            graph: self,
            next_e: self.vertex_to_first_edge[u],
        }
    }

    pub fn adj_list(&self, u: usize) -> AdjListIterator
    {
        AdjListIterator {
            graph: self,
            next_e: self.vertex_to_first_edge[u],
        }
    }

    pub fn edges<'a>(&'a self) -> impl Iterator<Item = (usize, usize)> + 'a
    {
        (0..self.num_v())
            .map(move |u| self.adj_list_with_edges(u).map(move |(_e, v)| (u, v)))
            .flatten()
    }
}

/// An iterator for convenient adjacency list traversal.
pub struct AdjListIteratorWithEdges<'a>
{
    graph: &'a Graph,
    next_e: Option<usize>,
}

impl<'a> Iterator for AdjListIteratorWithEdges<'a>
{
    type Item = (usize, usize);

    /// Produces an outgoing edge and vertex.
    fn next(&mut self) -> Option<Self::Item>
    {
        self.next_e.map(|e| {
            let v = self.graph.endp[e];
            self.next_e = self.graph.edge_to_next_edge[e];
            (e, v)
        })
    }
}

pub struct AdjListIterator<'a>
{
    graph: &'a Graph,
    next_e: Option<usize>,
}

impl<'a> Iterator for AdjListIterator<'a>
{
    type Item = usize;

    /// Produces an outgoing edge and vertex.
    fn next(&mut self) -> Option<Self::Item>
    {
        self.next_e.map(|e| {
            let v = self.graph.endp[e];
            self.next_e = self.graph.edge_to_next_edge[e];
            v
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

        assert_eq!(graph.vertex_to_first_edge, vec![None; 4]);
        assert_eq!(graph.edge_to_next_edge, vec![Some(0usize); 0]);
        assert_eq!(graph.endp, vec![0usize; 0]);

        graph.add_edge(0, 1);

        assert_eq!(graph.vertex_to_first_edge, vec![Some(0), None, None, None]);
        assert_eq!(graph.edge_to_next_edge, vec![None; 1]);
        assert_eq!(graph.endp, vec![1]);

        graph.add_edge(0, 2);

        assert_eq!(graph.vertex_to_first_edge, vec![Some(1), None, None, None]);
        assert_eq!(graph.edge_to_next_edge, vec![None, Some(0)]);
        assert_eq!(graph.endp, vec![1, 2]);
    }

    //cargo test test_edge_iterator -- --nocapture
    #[test]
    fn test_edge_iterator()
    {
        let mut graph = Graph::new(4, 4);
        graph.add_edge(2, 2);
        graph.add_edge(2, 3);
        graph.add_edge(1, 0);
        graph.add_edge(3, 0);
        graph.add_edge(3, 2);

        assert_eq!(
            graph.edges().collect::<Vec<_>>(),
            vec![(1, 0), (2, 3), (2, 2), (3, 2), (3, 0)]
        );
    }

}
