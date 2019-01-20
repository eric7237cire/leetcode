
//edge from source to destination

use std::cmp::max;
use std::cmp::min;
use bit_vec::BitVec;
use std::collections::VecDeque;
use std::{u64,usize};


struct Edge
{
    src: usize,
    dest: usize,

    cap: u64,
    residue: u64, //cap-flow

    //bool ignore;

    //flow = capacity - residue
}

impl Edge
{
    pub fn new(src: usize,
    dest: usize,

    cap: u64,
    residue: u64) -> Edge {
        Edge { src, dest, cap, residue }
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

struct Flow
{
    //V [ node idx ] = list of edge idxs originating from node
    V: Vec<Vec<usize>> ,
    E: Vec < Edge >,

    source: usize,
    sink: usize
}

const PREV_SOURCE:usize = usize::MAX-2;

impl Flow
{
    pub fn new(source: usize,
    sink: usize) -> Self
    {
        Self { source, sink, V: Vec::new(), E: Vec::new()}
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


	void setIgnoreNode(int nodeIdx, bool ignore)
	{
		for(int e = 0; e < V[nodeIdx].size(); ++e)
		{
			int eIdx = V[nodeIdx][e];
			E[ eIdx ].ignore = ignore;
		}
	}
*/
	pub fn add_edge(&mut self, src: usize, dest: usize, cap: u64)
	{
		let e = self.E.len();

		if max(src,dest) >= self.V.len() {
            self.V.resize(max(src, dest) + 1, Vec::new());
        }

		self.V[src].push(e);
		self.V[dest].push(e+1);

		self.E.push(Edge::new(src, dest, cap, cap));

		//Residual = 0, so backwards edge begins saturated at max flow
		self.E.push(Edge::new(dest, src, cap, 0));
	}

	/*
		prev[ vertex id ] =  the edge id of the edge used to go to previous node
	*/
	fn findAugPathMaxFlow(&self, prev: &Vec<usize>) -> u64
	{
		let mut canPush: u64 = u64::MAX;

		let mut nodeIdx = self.sink;

		//			printf("Finding maximum flow through augmenting path. Sink=%d\n", sink);

		while( prev[nodeIdx] != PREV_SOURCE ) //nodeIdx is not the source
		{
			//assert!(prev[nodeIdx] >= 0);

			canPush = min(canPush, self.E[ prev[nodeIdx] ].residue );

			nodeIdx = self.E[ prev[nodeIdx] ].src;

			//if (debug)
			//	printf("Can push %d.  Next node in aug path %d\n", canPush, nodeIdx);
		}

		return canPush;
	}

	fn updateViaAugPath(&mut self, prev: &Vec<usize>, flowAdded: u64)
	{
		let mut nodeIdx  = self.sink;

		while( prev[nodeIdx] != PREV_SOURCE ) //nodeIdx is not the source
		{
			//assert!(prev[nodeIdx] >= 0);

			self.E[ prev[nodeIdx] ].residue -= flowAdded;
			//assert!(self.E[ prev[nodeIdx] ].residue >= 0);

			//Because we added the edges in pairs xor will either add one or subtract one
            self.E[ prev[nodeIdx] ^ 1].residue += flowAdded;
			assert!( self.E[ prev[nodeIdx] ^ 1 ].residue <= self.E[ prev[nodeIdx] ^ 1 ].cap);
            
			debug!("Pushing {} flow at node {} edge ids {} and {} \n",
				flowAdded, nodeIdx, prev[nodeIdx], prev[nodeIdx] ^ 1);

			nodeIdx = self.E[ prev[nodeIdx] ].src;
		}

	}

	fn augment(&mut self) -> u64
	{
		let nNodes = self.V.len();
		let mut prev = vec![usize::MAX-1; nNodes];
        let mut seen : BitVec = BitVec::from_elem(nNodes, false);

		prev[self.source] = PREV_SOURCE;

		let mut q: VecDeque<usize> = VecDeque::new();

		q.push_back(self.source);
		seen.set(self.source, true);
		while let Some(nodeIdx) = q.pop_front()
		{
			assert!(seen[nodeIdx]);

			//if (debug) printf("Popped node %d\n", nodeIdx);
			//Sink?


		    //if (debug) printf("Looking at node %d.  Edges count %d\n", nodeIdx, V[nodeIdx].size());
			for i in 0..self.V[nodeIdx].len()
			{
				let edgeIdx = self.V[nodeIdx][i];
				let anEdge = &self.E[ edgeIdx ];

				let trgNodeIdx = anEdge.dest;

				debug!("edges id {} target {} flow {} capacity {} seen: {}\n", edgeIdx, trgNodeIdx,
					anEdge.cap - anEdge.residue, anEdge.cap, seen[trgNodeIdx] );

				if (anEdge.residue == 0) {
                    continue;
                }

				//if (anEdge.ignore)
				//	continue;



				if ( !seen[trgNodeIdx])
				{
					prev[trgNodeIdx] = edgeIdx;
					seen.set(trgNodeIdx, true);
					q.push_back(trgNodeIdx);
				}
			}
			//printf("Done\n");
		}

		if (seen[self.sink])
		{
			debug!("reached sink\n");

			let canPush = self.findAugPathMaxFlow(&prev);
			assert!(canPush > 0);

			self.updateViaAugPath( &prev, canPush );

			return canPush;
		}

		//printf("Return 0\n");
		return 0;
	}
}

#[cfg(test)]
mod test {

	use super::*;

static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;

use log::{Record, Level, Metadata, LevelFilter};
struct ConsoleLogger;

impl log::Log for ConsoleLogger {
  fn enabled(&self, metadata: &Metadata) -> bool {
     metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{}", record.args());
        }
    }

    fn flush(&self) {}
}


#[test]
fn test1()  {
	log::set_logger(&CONSOLE_LOGGER).unwrap();
    log::set_max_level(LevelFilter::Debug);

	debug!("Hello");
	println!("nth");

	let source = 0;
	let sink = 5;
	let mut flow = Flow::new(source,sink);

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
		total_flow+=dbg!(flow.augment());
		debug!("Total flow {}", total_flow);
	}
}

}