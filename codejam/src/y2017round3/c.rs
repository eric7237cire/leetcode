use super::super::util::input::*;
//use crate::algo::graph::cycles::simple_cycles;
//use crate::algo::graph::scc::strongly_connected_components;
use crate::algo::graph::DiGraph;
//use std::collections::HashMap;
use bit_set::BitSet;
//use bit_vec::BitVec;
//use std::collections::HashMap;
use std::io::Write;
use std::time::Instant;

/*

*/
pub fn solve_all_cases()
{
    let now = Instant::now();

    let mut reader = InputReader::new();
    let t = reader.read_int();

    for case in 1..=t {
        let (F, P) = reader.read_tuple_2();
        let mut G = DiGraph::new();
        for f in 1..=F {
            G.add_vertex(f);
        }
        let P = (0..P)
            .map(|_| {
                let (f1, f2) = reader.read_tuple_2();
                G.add_edge(f1, f2);
                (f1, f2)
            })
            .collect::<Vec<_>>();

        print!("{}", solve(case, &G, &P, F));
    }

    let duration = now.elapsed();
    let secs = f64::from(duration.as_secs() as u32) + f64::from(duration.subsec_nanos()) / 1e9f64;
    let _ = writeln!(
        ::std::io::stderr(),
        "\nElapsed time {:.2} second(s)\n",
        secs
    );
}

fn solve(case_no: u32, G: &DiGraph, P: &Vec<(usize, usize)>, F: usize) -> String
{
    debug!("\n\n\nSolving case {}", case_no);



    format!(
        "Case #{}: \n",
        case_no)

}
