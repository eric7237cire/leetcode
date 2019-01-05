use super::super::util::input::*;
//use itertools::Itertools;
//use std::fmt;
//use std::io::stdin;
//use std::iter::FromIterator;
use std::f64;
use std::u16;
use std::cmp::Ordering;

struct Horse
{
    E: u32, //the maximum total distance, in kilometers, the horse in the i-th city can go and
    S: u16, // Si, the constant speed, in kilometers per hour, at which the horse travels.
}

type Distance = u32;
type CityIndex = usize;

pub fn solve_all_cases()
{
    //let mut children: Vec<thread::JoinHandle<_>> = vec![];
    let mut reader = InputReader::new();
    let t = reader.read_int();

    for case in 1..=t
    {
        //N, R, O(RY), Y, G(YB), B, and V(RB).
        let (N, Q) = reader.read_tuple_2::<u8, u8>();
        let horses: Vec<_> = (0..N)
            .map(|_| reader.read_tuple_2::<Distance, u16>())
            .map(|tp| Horse { E: tp.0, S: tp.1 })
            .collect();
        let city_dist: Vec<_> = (0..N)
            .map(|_| {
                reader.read_int_line::<i32>().iter().map(|&d| {
                    if d < 0
                    {
                        None
                    }
                    else
                    {
                        Some(d as Distance)
                    }
                }).collect()
            })
            .collect();
        let queries: Vec<_> = (0..Q)
            .map(|_| reader.read_tuple_2::<CityIndex, CityIndex>())
            .collect();
        //  children.push(thread::spawn(move || -> String { solve(case, &input) }));
        print!("{}", solve(case, &horses, &city_dist, &queries));
    } /*
      for child in children
      {
          print!("{}", child.join().unwrap());
      }*/
}

use std::collections::BinaryHeap;

#[allow(non_snake_case)]
#[derive(Copy, Clone)]
struct Node
{
    time: f64,
    city_horse_index: CityIndex,
}

impl Node
{
    fn split_index(N: CityIndex, city_horse_index: CityIndex) -> (CityIndex, CityIndex)
    {
        let city = city_horse_index / N;
        let horse = city_horse_index % N;
        (city, horse)
    }

    fn to_index(N: CityIndex, city: CityIndex, horse: CityIndex) -> CityIndex
    {
        assert!(horse < N);
        city * N + horse
    }
}

impl Eq for Node {}


impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.city_horse_index == other.city_horse_index &&
            self.time.partial_cmp(&other.time).unwrap() == Ordering::Equal
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .time
            .partial_cmp(&self.time).unwrap()
            .then_with(|| self.city_horse_index.cmp(&other.city_horse_index))
    }
}

#[allow(non_snake_case)]
fn solve(
    case_no: u32,
    horses: &Vec<Horse>,
    city_dist: &Vec<Vec<Option<Distance>>>,
    queries: &Vec<(CityIndex, CityIndex)>,
) -> String
{
    debug!("Solving case {}", case_no);
    format!("Case #{}: {}\n", case_no,
    queries.iter().map(|q|
        solve_query(horses, city_dist, q.0 - 1, q.1 - 1)
    ).map(|f| f.to_string()).collect::<Vec<_>>().join(" "))
}
fn solve_query(
    horses: &Vec<Horse>,
    city_dist: &Vec<Vec<Option<Distance>>>,
    start_city: CityIndex,
    stop_city: CityIndex,
) -> f64
{
    debug!("Solving query from {} to {}", start_city+1, stop_city+1);

    let N = horses.len() as CityIndex;
    let NODE_COUNT = N * N;

    // dist[node] = current shortest distance from `start` to `node`
    let mut shortest_time: Vec<_> = (0..NODE_COUNT).map(|_| f64::MAX).collect();
    let mut prev: Vec<Option<CityIndex>> = vec![None; NODE_COUNT as usize];

    let mut heap = BinaryHeap::new();

    let start = Node::to_index(N, start_city, start_city);

    // We're at `start`, with a zero time
    shortest_time[start as usize] = 0f64;
    heap.push(Node {
        time: 0f64,
        city_horse_index: start,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(current_node) = heap.pop()
    {
        let Node{time, city_horse_index} = current_node;
        let (city_index, horse_index) = Node::split_index(N, city_horse_index);

        if city_index == stop_city
        {
            return time;
        }

        // Important as we may have already found a better way
        if time > shortest_time[city_horse_index]
        {
            continue;
        }

        assert_eq!(time, shortest_time[city_horse_index]);

        //follow prev nodes to find how long we have gone
        let mut dist_travelled_with_current_horse = 0;
        let mut p = city_horse_index;
        while Node::split_index(N, p).0 != horse_index
        {
            let (p_city_index, _p_horse_index) = Node::split_index(N, p);
            let pp = prev[p].unwrap();
            let (pp_city_index, _pp_horse_index) = Node::split_index(N, pp);
            dist_travelled_with_current_horse += city_dist[pp_city_index][p_city_index].unwrap();
            p = pp;
        }

        let dis_remaining_with_current_horse = horses[horse_index].E - dist_travelled_with_current_horse;

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for (next_city_index, dist) in city_dist[city_index].iter().enumerate()
        {
            if dist.is_none()
            {
                continue;
            }

            let dist = dist.unwrap();

            if dist > dis_remaining_with_current_horse
            {
                continue;
            }

            let time_taken = time + dist as f64 / horses[horse_index].S as f64;

            for change_horses in 0..2
            {
                let next_horse_index = if change_horses == 0
                {
                    horse_index
                }
                else
                {
                    next_city_index
                };

                let next = Node {
                    time: time_taken,
                    city_horse_index: Node::to_index(N, next_city_index, next_horse_index),
                };

                // If so, add it to the frontier and continue
                // We want all paths, so == cost is OK
                if next.time <= shortest_time[next.city_horse_index]
                {
                    heap.push(next);
                    // Relaxation, we have now found a better way
                    shortest_time[next.city_horse_index] = next.time;

                    // Update prev
                    prev[next.city_horse_index] = Some(city_horse_index);

                }
            }
        }
    }

    /*
            println!("distance from start {} to end {} is {}", start, stop, dist[stop]);
            println!("Adj list is {:?}", adj_list);
            for (i, item) in prev.iter().enumerate() {
                println!("Prev for node# {} / {} is {:?}",
                    i,
                    word_list[i].clone(),
                    vec_idx_to_str(&prev[i], &word_list)
                );
                println!("Dist for node# {} / {} is {:?}",
                    i,
                    word_list[i].clone(),
                    dist[i]
                );
            }
    */
    -1f64
}
