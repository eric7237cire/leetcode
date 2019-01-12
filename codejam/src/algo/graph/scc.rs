//https://stackoverflow.com/questions/46511682/non-recursive-version-of-tarjans-algorithm

//https://www.geeksforgeeks.org/iterative-depth-first-traversal/

//! Graph connectivity structures.
use super::Graph;

//Directed only

/// Helper struct that carries data needed for the depth-first searches in
/// ConnectivityGraph's constructor.
struct ConnectivityData
{
    time: usize,
    vis: Vec<usize>,
    low: Vec<usize>,
    v_stack: Vec<usize>,

}

impl ConnectivityData
{
    fn new(num_v: usize) -> Self
    {
        Self {
            time: 0,
            vis: vec![0; num_v],
            low: vec![0; num_v],
            v_stack: Vec::new(),
        }
    }

    fn visit(&mut self, u: usize)
    {
        self.time += 1;
        self.vis[u] = self.time;
        self.low[u] = self.time;
        self.v_stack.push(u);
    }

    fn lower(&mut self, u: usize, val: usize)
    {
        if self.low[u] > val {
            self.low[u] = val
        }
    }
}

/// Represents the decomposition of a graph into any of its constituent parts:
///
/// - Connected components (CC),
/// - Strongly connected components (SCC),
/// - 2-edge-connected components (2ECC),
/// - 2-vertex-connected components (2VCC)
///
/// Multiple-edges and self-loops are correctly handled.
pub struct ConnectivityGraph<'a>
{
    // Immutable graph, frozen for the lifetime of the ConnectivityGraph object.
    pub graph: &'a Graph,
    /// ID of a vertex's CC, SCC or 2ECC, whichever applies.
    pub cc: Vec<usize>,

    /// Total number of CCs, SCCs or 2ECCs, whichever applies.
    pub num_cc: usize,

}

impl<'a> ConnectivityGraph<'a>
{
    /// Computes CCs (connected components), SCCs (strongly connected
    /// components), 2ECCs (2-edge-connected components), and/or 2VCCs
    /// (2-vertex-connected components), depending on the parameter and graph:
    /// - is_directed == true on directed graph: SCCs in rev-topological order
    /// - is_directed == true on undirected graph: CCs
    /// - is_directed == false on undirected graph: 2ECCs and 2VCCs
    /// - is_directed == false on directed graph: undefined behavior
    pub fn new(graph: &'a Graphl) -> Self
    {
        let mut connect = Self {
            graph,
            cc: vec![0; graph.num_v()],
            num_cc: 0,
        };
        let mut data = ConnectivityData::new(graph.num_v());
        for u in 0..graph.num_v() {
            if data.vis[u] == 0 {

                    connect.scc(u, &mut data);

            }
        }
        connect
    }

    //Tarjan's algorithm
    fn scc(&mut self, u: usize, data: &mut ConnectivityData)
    {
        data.visit(u);
        for (_, v) in self.graph.adj_list(u) {
            if data.vis[v] == 0 {
                self.scc(v, data);
            }
            if self.cc[v] == 0 {
                data.lower(u, data.low[v]);
            }
        }
        if data.vis[u] == data.low[u] {
            self.num_cc += 1;
            while let Some(v) = data.v_stack.pop() {
                self.cc[v] = self.num_cc;
                if v == u {
                    break;
                }
            }
        }
    }




}

#[cfg(test)]
mod test
{
    use super::*;



    #[test]
    fn test_scc()
    {
        let mut graph = Graph::new(3, 6);
        graph.add_undirected_edge(0, 1);
        graph.add_undirected_edge(1, 2);
        graph.add_undirected_edge(1, 2);

        let cg = ConnectivityGraph::new(&graph, false);
        let bridges = (0..graph.num_e())
            .filter(|&e| cg.is_cut_edge(e))
            .collect::<Vec<_>>();
        let articulation_points = (0..graph.num_v())
            .filter(|&u| cg.is_cut_vertex(u))
            .collect::<Vec<_>>();

        assert_eq!(bridges, vec![0, 1]);
        assert_eq!(articulation_points, vec![1]);
    }
}
