use crate::algo::graph::DiGraph;
use bit_vec::BitVec;

impl DiGraph
{
    fn dfs(&self, v: usize) -> DfsIterator
    {
        // Create a stack for DFS
        let mut stack: Vec<usize> = Vec::new();

        // Push the current source node.
        stack.push(v);

        DfsIterator {
            graph: self,
            visited: BitVec::from_elem(self.max_v(), false),
            stack,
        }
    }
}
pub struct DfsIterator<'a>
{
    graph: &'a DiGraph,
    //is vertex visited
    visited: BitVec,
    //stack of vertices
    stack: Vec<usize>,
}

impl<'a> Iterator for DfsIterator<'a>
{
    type Item = usize;

    /// Returns next vertex in the DFS
    fn next(&mut self) -> Option<Self::Item>
    {
        let mut r = None;

        //Code translated/adapted from https://www.geeksforgeeks.org/iterative-depth-first-traversal/
        while let Some(s) = self.stack.pop() {
            // Stack may contain same vertex twice. So
            // we need to print the popped item only
            // if it is not visited.
            if !self.visited[s] {
                r = Some(s);
                self.visited.set(s, true);
                // Get all adjacent vertices of the popped vertex s
                // If a adjacent has not been visited, then puah it
                // to the stack.
                self.stack.extend(self.graph.edges_from(s));
            }

            if r != None {
                return r;
            }
        }

        None
    }
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn test_dfs()
    {
        let mut graph = DiGraph::new();
        graph.add_edge(0, 2);
        graph.add_edge(2, 0);
        graph.add_edge(1, 2);
        graph.add_edge(0, 1);
        graph.add_edge(3, 3);
        graph.add_edge(2, 3);

        //start at 2;  -- 2 0 1 3

        let dfs_search = graph.dfs(2).collect::<Vec<_>>();
        assert_eq!(dfs_search, vec![2, 3, 0, 1]);
    }

    #[test]
    fn test_dfs2()
    {
        let mut graph = DiGraph::new();
        graph.add_edge(0, 2);
        graph.add_edge(2, 1);
        graph.add_edge(1, 0);
        graph.add_edge(0, 3);
        graph.add_edge(3, 4);
        graph.add_edge(4, 0);

        let dfs_search = graph.dfs(0).collect::<Vec<_>>();
        assert_eq!(dfs_search, vec![0, 3, 4, 2, 1]);
    }
}
