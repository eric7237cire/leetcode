use super::super::util::input::*;
use crate::algo::graph::cycles::simple_cycles;
use crate::algo::graph::DiGraph;
use std::collections::HashMap;
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

        print!("{}", solve(case, &G, &P));
    }

    let duration = now.elapsed();
    let secs = f64::from(duration.as_secs() as u32) + f64::from(duration.subsec_nanos()) / 1e9f64;
    let _ = writeln!(
        ::std::io::stderr(),
        "\nElapsed time {:.2} second(s)\n",
        secs
    );
}

fn solve(case_no: u32, G: &DiGraph, P: &Vec<(usize, usize)>) -> String
{
    debug!("Solving case {}", case_no);

    let mut G = G.clone();
    //add reverse edges
    for &(u, v) in P.iter() {
        G.add_edge(v, u);
    }

    let cycles = simple_cycles(&G);

    debug!("Graph is {:?}, Cycles are {:?}", G.edges().collect::<Vec<_>>(), cycles);

    let mut news_values: HashMap<(usize,usize), i64> = HashMap::new();

    for (cycle_index, cycle) in cycles.iter().enumerate() {
        let mut c_it = cycle.iter().cycle();
        for _ in 0..cycle.len() {
            let edge = (*c_it.next().unwrap(), *c_it.next().unwrap());
            *news_values.entry(edge).or_insert(0i64) += cycle_index as i64 + 1;
        }
    }

    let ans = P.iter().map( |&fe| {
        let mut sum = 0;
        if let Some(pos) = news_values.get(&fe) {
            sum += pos;
        }
        if let Some(neg) = news_values.get(&(fe.1, fe.0)) {
            sum -= neg;
        }
        sum.to_string()
    }).collect::<Vec<String>>().join(" ");
    //debug!("G {:?} {}", digits, G,);

    format!("Case #{}: {}\n", case_no, ans)
}
