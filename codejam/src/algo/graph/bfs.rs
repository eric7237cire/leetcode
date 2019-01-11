use super::graph::Graph;
use bit_vec::BitVec;
use std::collections::VecDeque;

impl Graph
{
    fn bfs(&self, v: usize) -> BfsIterator
    {
        let mut queue: VecDeque<usize> = VecDeque::new();
        let mut visited = BitVec::from_elem(self.num_v(), false);

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
    graph: &'a Graph,
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

        for (_e, u) in self.graph.adj_list(r) {
            if !self.visited[u] {
                self.visited.set(u, true);
                self.queue.push_back(u);
            }
        }

        return Some(r);
    }
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn test_bfs()
    {
        let mut graph = Graph::new(4, 8);
        graph.add_edge(0, 2);
        graph.add_edge(2, 0);
        graph.add_edge(1, 2);
        graph.add_edge(0, 1);
        graph.add_edge(3, 3);
        graph.add_edge(2, 3);

        //start at 2;  -- 2 0 1 3

        let bfs_search = graph.bfs(2).collect::<Vec<_>>();
        assert_eq!(bfs_search, vec![2, 3, 0, 1]);
    }

    #[test]
    fn test_bfs2()
    {
        let mut graph = Graph::new(5, 8);
        graph.add_edge(0, 2);
        graph.add_edge(2, 1);
        graph.add_edge(1, 0);
        graph.add_edge(0, 3);
        graph.add_edge(3, 4);
        graph.add_edge(4, 0);

        let bfs_search = graph.bfs(0).collect::<Vec<_>>();
        assert_eq!(bfs_search, vec![0, 3, 2, 4, 1]);
    }
}
