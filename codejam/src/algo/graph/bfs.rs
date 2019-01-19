use super::directed_graph::DiGraph;
use bit_vec::BitVec;
use std::collections::VecDeque;

impl DiGraph
{
    pub fn bfs(&self, v: usize) -> BfsIterator
    {
        let mut queue: VecDeque<usize> = VecDeque::new();
        let mut visited = BitVec::from_elem(self.max_v() + 1, false);

        queue.push_back(v);
        visited.set(v, true);

        BfsIterator {
            graph: self,
            visited,
            queue,
        }
    }
}
pub struct BfsIterator<'a>
{
    graph: &'a DiGraph,
    //is vertex visited
    visited: BitVec,
    queue: VecDeque<usize>,
}

impl<'a> Iterator for BfsIterator<'a>
{
    type Item = usize;

    /// Returns next vertex in the DFS
    fn next(&mut self) -> Option<Self::Item>
    {
        if self.queue.is_empty() {
            return None;
        }

        let r = self.queue.pop_front().unwrap();

        //Code translated/adapted from https://www.geeksforgeeks.org/breadth-first-search-or-bfs-for-a-graph/

        for u in self.graph.edges_from(r) {
            if !self.visited[u] {
                self.visited.set(u, true);
                self.queue.push_back(u);
            }
        }

        Some(r)
    }
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn test_bfs()
    {
        let mut graph = DiGraph::new();
        graph.add_edge(0, 2);
        graph.add_edge(2, 0);
        graph.add_edge(1, 2);
        graph.add_edge(0, 1);
        graph.add_edge(3, 3);
        graph.add_edge(2, 3);

        //start at 2;  -- 2 0 1 3

        let mut bfs_search = graph.bfs(2).collect::<Vec<_>>();
        assert_eq!(bfs_search[3], 1);

        bfs_search.sort();
        assert_eq!(bfs_search, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_bfs2()
    {
        let mut graph = DiGraph::new();
        graph.add_edge(0, 2);
        graph.add_edge(2, 1);
        graph.add_edge(1, 0);
        graph.add_edge(0, 3);
        graph.add_edge(1, 4);

        let mut bfs_search = graph.bfs(0).collect::<Vec<_>>();

        assert_eq!(bfs_search[4], 4);

        bfs_search.sort();
        assert_eq!(bfs_search, vec![0, 1, 2, 3, 4]);
    }
}
