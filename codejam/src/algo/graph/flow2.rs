//edge from source to destination

use bit_vec::BitVec;
use std::cmp::max;
use std::cmp::min;
use std::collections::VecDeque;
use std::{u64, usize};

pub struct Edge
{
    pub src: usize,
    pub dest: usize,

    pub cap: u64,
    pub residue: u64, //cap-flow

    ignore: bool, //flow = capacity - residue
}

impl Edge
{
    pub fn new(src: usize, dest: usize, cap: u64, residue: u64) -> Edge
    {
        Edge {
            src,
            dest,
            cap,
            residue,
            ignore: false,
        }
    }
}

/*
template<typename u64>
ostream& operator<<(ostream& os, const edge<u64>& e)
{
    os <<  e.src << " --> " << e.dest
        << " flow " << e.cap - e.residue << " / " << e.cap ;

    return os;
}*/

pub struct Flow
{
    //V [ node idx ] = list of edge idxs originating from node
    pub V: Vec<Vec<usize>>,
    pub E: Vec<Edge>,

    pub source: usize,
    pub sink: usize,

    prev: Vec<usize>,
}

const PREV_SOURCE: usize = usize::MAX - 2;
const PREV_NONE: usize = usize::MAX - 1;

impl Flow
{
    pub fn new(source: usize, sink: usize, est_num_vertices: usize) -> Self
    {
        Self {
            source,
            sink,
            V: Vec::with_capacity(est_num_vertices),
            E: Vec::with_capacity(est_num_vertices * 2),
            prev: Vec::with_capacity(est_num_vertices),
        }
    }

    //set flow back to 0
    /*void resetFlow()
            {
                for(int i = 0; i < E.size(); ++i)
                {
                    if (i % 2 == 0)
                        E[i].residue = E[i].cap;
                    else
                        E[i].residue = 0;
                }

            }
    */

    pub fn reset_edge_flow(&mut self, edge_idx: usize)
    {
        if edge_idx % 2 == 0 {
            self.E[edge_idx].residue = self.E[edge_idx].cap;
            self.E[edge_idx ^ 1].residue = 0;
        } else {
            self.E[edge_idx].residue = 0;
            self.E[edge_idx ^ 1].residue = self.E[edge_idx].cap;
        }
    }

    pub fn remove_edge(&mut self, edge_idx: usize)
    {
        {
            let mut edge = &mut self.E[edge_idx];
            edge.residue = 0;
            edge.cap = 0;
            edge.ignore = true;
        }
        {
            self.remove_edge_from_vertex(edge_idx, self.E[edge_idx].src);
            self.remove_edge_from_vertex(edge_idx, self.E[edge_idx].dest);
        }
        {
            let mut edge = &mut self.E[edge_idx ^ 1];
            edge.residue = 0;
            edge.cap = 0;
            edge.ignore = true;
        }
    }

    fn remove_edge_from_vertex(&mut self, edge_idx: usize, vertex_idx: usize)
    {
        self.V[vertex_idx].retain(|e| *e != edge_idx && *e != edge_idx ^ 1);
    }

    pub fn setIgnoreNode(&mut self, nodeIdx: usize, ignore: bool)
    {
        for &e in self.V[nodeIdx].iter() {
            let eIdx = self.V[nodeIdx][e];
            self.E[eIdx].ignore = ignore;
        }
    }

    pub fn add_edge(&mut self, src: usize, dest: usize, cap: u64)
    {
        let e = self.E.len();

        if max(src, dest) >= self.V.len() {
            self.V.resize(max(src, dest) + 1, Vec::new());
        }

        self.V[src].push(e);
        self.V[dest].push(e + 1);

        self.E.push(Edge::new(src, dest, cap, cap));

        //Residual = 0, so backwards edge begins saturated at max flow
        self.E.push(Edge::new(dest, src, cap, 0));
    }

    /*
        prev[ vertex id ] =  the edge id of the edge used to go to previous node
    */
    fn findAugPathMaxFlow(&self) -> u64
    {
        let mut canPush: u64 = u64::MAX;
        let prev = &self.prev;

        let mut nodeIdx = self.sink;

        //			printf("Finding maximum flow through augmenting path. Sink=%d\n", sink);

        while (prev[nodeIdx] != PREV_SOURCE)
        //nodeIdx is not the source
        {
            let prev_edge_idx = prev[nodeIdx];
            assert!(prev_edge_idx != PREV_NONE);
            //assert!(prev[nodeIdx] >= 0);

            let dual_edge_idx = prev_edge_idx ^ 1;
            let prev_edge = &self.E[prev_edge_idx];
            let dual_edge = &self.E[dual_edge_idx];

            assert_eq!(dual_edge.cap, prev_edge.cap);
            assert_eq!(dual_edge.residue + prev_edge.residue, prev_edge.cap);

            canPush = min(canPush, prev_edge.residue);

            nodeIdx = prev_edge.src;

            //if (debug)
            //	printf("Can push %d.  Next node in aug path %d\n", canPush, nodeIdx);
        }

        return canPush;
    }

    fn updateViaAugPath(&mut self, flowAdded: u64)
    {
        let prev = &self.prev;
        let mut nodeIdx = self.sink;

        while (prev[nodeIdx] != PREV_SOURCE)
        //nodeIdx is not the source
        {
            //assert!(prev[nodeIdx] >= 0);
            assert_eq!(self.E[prev[nodeIdx]].dest, nodeIdx);

            self.E[prev[nodeIdx]].residue -= flowAdded;
            //assert!(self.E[ prev[nodeIdx] ].residue >= 0);

            //Because we added the edges in pairs xor will either add one or subtract one
            let reverse_edge = &mut self.E[prev[nodeIdx] ^ 1];
            reverse_edge.residue += flowAdded;
            assert!(reverse_edge.residue <= reverse_edge.cap);
            assert_eq!(reverse_edge.src, nodeIdx);

            debug!(
                "Pushing {} flow at node {}->node {} edge ids {} and {} \n",
                flowAdded,
                reverse_edge.dest,
                reverse_edge.src,
                prev[nodeIdx],
                prev[nodeIdx] ^ 1
            );

            nodeIdx = self.E[prev[nodeIdx]].src;
        }
    }

    /// Implements https://en.wikipedia.org/wiki/Edmonds%E2%80%93Karp_algorithm
    /// because a BFS is used
    pub fn augment(&mut self) -> u64
    {
        let nNodes = self.V.len();
        let mut prev = &mut self.prev;
        prev.resize(nNodes, PREV_NONE);
        let mut seen: BitVec = BitVec::from_elem(nNodes, false);

        prev[self.source] = PREV_SOURCE;

        let mut q: VecDeque<usize> = VecDeque::new();

        q.push_back(self.source);
        seen.set(self.source, true);

        let mut iteration_count = 0;

        while let Some(nodeIdx) = q.pop_front() {
            iteration_count += 1;

            assert!(seen[nodeIdx]);

            //if (debug) printf("Popped node %d\n", nodeIdx);
            //Sink?

            //if (debug) printf("Looking at node %d.  Edges count %d\n", nodeIdx, V[nodeIdx].size());
            for i in 0..self.V[nodeIdx].len() {
                let edgeIdx = self.V[nodeIdx][i];
                let anEdge = &self.E[edgeIdx];

                let trgNodeIdx = anEdge.dest;

                /*
                                debug!(
                                    "edges id {} target {} flow {} capacity {} seen: {}\n",
                                    edgeIdx,
                                    trgNodeIdx,
                                    anEdge.cap - anEdge.residue,
                                    anEdge.cap,
                                    seen[trgNodeIdx]
                                );
                */
                if (anEdge.residue == 0) {
                    continue;
                }

                //if (anEdge.ignore)
                //	continue;

                if (!seen[trgNodeIdx]) {
                    prev[trgNodeIdx] = edgeIdx;
                    seen.set(trgNodeIdx, true);
                    q.push_back(trgNodeIdx);
                }
            }
            //printf("Done\n");
        }

        if iteration_count > 10000 {
            //println!("Iteration count {}", iteration_count);
        }

        if (seen[self.sink]) {
            debug!("reached sink\n");

            let canPush = self.findAugPathMaxFlow();
            assert!(canPush > 0);

            self.updateViaAugPath(canPush);

            return canPush;
        }

        //printf("Return 0\n");
        return 0;
    }
}

#[cfg(test)]
mod test
{

    use super::*;

    static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;

    use log::{Level, LevelFilter, Metadata, Record};
    struct ConsoleLogger;

    impl log::Log for ConsoleLogger
    {
        fn enabled(&self, metadata: &Metadata) -> bool
        {
            metadata.level() <= Level::Debug
        }

        fn log(&self, record: &Record)
        {
            if self.enabled(record.metadata()) {
                println!("{}", record.args());
            }
        }

        fn flush(&self)
        {
        }
    }

    #[test]
    fn test1()
    {
        log::set_logger(&CONSOLE_LOGGER).unwrap();
        log::set_max_level(LevelFilter::Debug);

        debug!("Hello");
        println!("nth");

        let source = 0;
        let sink = 5;
        let mut flow = Flow::new(source, sink, 10);

        flow.add_edge(0, 1, 10);
        flow.add_edge(0, 2, 10);
        flow.add_edge(1, 2, 2);
        flow.add_edge(1, 4, 8);
        flow.add_edge(1, 3, 4);
        flow.add_edge(2, 4, 9);
        flow.add_edge(4, 3, 6);
        flow.add_edge(3, 5, 10);
        flow.add_edge(4, 5, 10);

        let mut total_flow = 0;
        for _ in 0..6 {
            total_flow += dbg!(flow.augment());
            debug!("Total flow {}", total_flow);
        }
    }

}
