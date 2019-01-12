// maybe like Problem D. Descending in the Dark round 2 2012

/*
Bipartite matching
Grid
BFS
Cycles
Hard
*/
use super::super::algo::graph::flow::*;
use super::super::algo::graph::*;
use super::super::util::grid::constants::*;
use super::super::util::grid::{Grid, GridCoord, GridRowColVec};
use super::super::util::input::*;
//use std::thread;
use std::default::Default;
use std::fmt;
use std::fmt::{Display, Formatter};
use bimap::BiMap;
use bit_vec::BitVec;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::Write;
use std::time::{Instant};
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use indexmap::IndexSet;

pub fn solve_all_cases()
{
    let now = Instant::now();

    //let mut children: Vec<thread::JoinHandle<_>> = vec![];
    let pool = ThreadPool::new(6);

    let (tx, rx) = channel();

    let mut reader = InputReader::new();
    let t = reader.read_int();

    for case in 1..=t {
        let (C, R, M) = reader.read_tuple_3::<usize, usize, usize>();
        let mut grid: Grid<Tile> = Grid::new(R, C);
        for r in 0..R {
            let row = reader.read_chars(C);
            for (c, t) in row.iter().enumerate() {
                grid[(r, c)] = Tile::from(*t);
            }
        }

        let tx = tx.clone();
        pool.execute(move || {
            let now = Instant::now();
            let _ = writeln!(::std::io::stderr(), "Starting {} of {} ", case, t);
            let s = solve(case, &mut grid, M);
            tx.send((case,s)).expect("Channel is there");

            let duration = now.elapsed();
            let secs = duration.as_secs() as f64 + duration.subsec_nanos() as f64 / 1e9f64;
            let _ = writeln!(::std::io::stderr(), "Finished #{} in {:.2} second(s)", case, secs);


        });

    }

    let mut output = rx.iter().take(t as usize).collect::<Vec<_>>();
    output.sort();
    for (_,s) in output  {
        print!("{}", s);
    }

    let duration = now.elapsed();
    let secs = duration.as_secs() as f64 + duration.subsec_nanos() as f64 / 1e9f64;
    let _ = write!(::std::io::stderr(), "\nElapsed time {:.2} second(s)\n", secs);
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tile
{
    Empty,
    Building,
    Soldier,
    Turret,
}

use self::Tile::*;


impl Tile
{
    fn to_char(self) -> char
    {
        match self {
            Empty => '.',
            Building => '#',
            Soldier => 'S',
            Turret => 'T',
        }
    }
}

impl From<char> for Tile
{
    fn from(item: char) -> Self
    {
        match item {
            '.' => Empty,
            '#' => Building,
            'S' => Soldier,
            'T' => Turret,
            _ => panic!("Character not recognized: {}", item),
        }
    }
}
impl Display for Tile
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        write!(f, "{}", self.to_char())
    }
}

impl Default for Tile
{
    fn default() -> Tile
    {
        Empty
    }
}

//problem specific code
fn reachable(grid: &Grid<Tile>, location: GridCoord) -> HashSet<GridRowColVec>
{
    let mut r = HashSet::new();
    //debug!("\nTracing {} starting at {}", location, direction);

    for direction in DIRECTIONS.iter() {
        let mut loc: GridRowColVec = location.convert();

        for _ in 0..=grid.R + grid.C {
            loc += *direction;

            if let Some(tile) = grid.get_value(loc) {
                match *tile {
                    Building => {
                        break;
                    }
                    _ => {
                        r.insert(loc);
                    }
                };
            } else {
                break;
            }
        }
    }

    r
}

/*
impl<L, R> FromIterator<(L, R)> for BiMap<L, R>
{
    fn from_iter<I: IntoIterator<Item = (L, R)>>(iter: I) -> Self
    {
        let mut c = BiMap::new();

        for i in iter {
            c.insert(i.0, i.1);
        }

        c
    }
}*/

fn solve<'a>(case_no: u32, grid: &mut Grid<Tile>, M_soldier_limit: usize) -> String
{

    debug!(
        "Solving case {}\nM={}\n{}\n",
        case_no, M_soldier_limit, grid
    );

    //original solider & turret index to location map
    let S_map = grid
        .filter_by_val(&Soldier)
        .enumerate()
        .collect::<BiMap<_, _>>();
    let turret_locations = grid
        .filter_by_val(&Turret).collect::<Vec<_>>();

    //precalucate what squares a turret can reach
    let turret_reachable_squares_list = turret_locations
        .iter()
        .map(|t_loc| reachable(&grid, *t_loc))
        .collect::<Vec<_>>();

    let T_map = turret_locations.into_iter()
        .enumerate()
        .collect::<BiMap<_, _>>();

    let S = grid.filter_by_val(&Soldier).count();
    let T = grid.filter_by_val(&Turret).count();



    //Construct the initial Graph

    let G_edges = build_graph(&grid, false, M_soldier_limit, &S_map, &T_map, &turret_reachable_squares_list);

    let mut G = FlowGraph::new(2 + S + T, 4);

    for uv in G_edges {
        G.add_edge(uv.0, uv.1, 1, 1);
    }

    let source = S + T;
    let sink = S + T + 1;

    let vertex_to_string = |v: usize| match v {
        s if s < S => format!("Soldier #{} ({})", s + 1, *S_map.get_by_left(&s).unwrap()),
        t if t >= S && t < S + T => format!(
            "Turret #{} ({})",
            t - S + 1,
            *T_map.get_by_left(&(t - S)).unwrap()
        ),
        v if v == sink => "Sink".to_string(),
        _source => "Source".to_string(),
    };

    //BFS for each soldier

    //will be in left to right order, then top down order

    //Now find max matching of G (G has an edge from soldier s to turret t if and only if soldier s can destroy turret t after all other turrets have been destroyed)
    for s in 0..S {
        G.add_edge(source, s, 1, 1);
    }

    for t in S..S + T {
        G.add_edge(t, sink, 1, 1);
    }

    let (R, flow) = G.dinic(source, sink);

    let mut ans = format!("Case #{}: {}\n", case_no, R);


    //Compute initial matching
    let mut M = flow
        .iter()
        .enumerate()
        .filter(|&(_e, f)| *f > 0)
        //map to u->v
        .map(|(e, _f)| (G.graph.endp[e ^ 1], G.graph.endp[e]))
        //leave out source and sink nodes
        .filter(|&(u, v)| u != source && v != sink)
        .collect::<Vec<_>>();

    debug!(
        "Edges in M initial matching=\n{}\n",
        M.iter()
            .map(|&(u, v)| format!("{}->{}", vertex_to_string(u), vertex_to_string(v)))
            .collect::<Vec<_>>()
            .join("\n")
    );

    let mut r = R;

    while r > 0 {
        //Let us define the graph G' with the same nodes as G, but an edge between soldier s and turret t only exists in G' if s can destroy t with the other turrets active
        let Gprime = build_graph(&grid, true,
                                 M_soldier_limit, &S_map, &T_map, &turret_reachable_squares_list);

        //Now build graph H
        let mut H = Graph::new(S + T, 4);

        let soldiers_in_m = M.iter().map(|&(s, _t)| s).collect::<Vec<_>>();

        for &(s, t) in Gprime.iter() {
            if soldiers_in_m.contains(&s) {
                H.add_edge(s, t);
            }
        }
        for &(s, t) in M.iter() {
            H.add_edge(t, s);
        }

        debug!(
            "Current matching M =\n{}\n",
            M.iter()
                .map(|&(u, v)| format!("{}->{}", vertex_to_string(u), vertex_to_string(v)))
                .collect::<Vec<_>>()
                .join("\n")
        );

        debug!(
            "Edges in G'=\n{}\n",
            Gprime
                .iter()
                .map(|&(u, v)| format!("{}->{}", vertex_to_string(u), vertex_to_string(v)))
                .collect::<Vec<_>>()
                .join("\n")
        );

        debug!(
            "Edges in H=\n{}\n",
            H.edges()
                .map(|(u, v)| format!("{}->{}", vertex_to_string(u), vertex_to_string(v)))
                .collect::<Vec<_>>()
                .join("\n")
        );

        let turrets_in_M = M.iter().map(|&(_s, t)| t).collect::<Vec<_>>();
        //find an edge (s,t') where t' is not in m
        let st_prime = Gprime
            .iter()
            .filter(|&(_s, t)| !turrets_in_M.contains(t))
            .next();

        if !st_prime.is_none() {
            let &(s, t) = st_prime.unwrap();
            debug!("Found (s,t') s={} t'={}", s, t - S);
            ans += &format!("{} {}\n", s + 1, t - S + 1);

            grid[*S_map.get_by_left(&s).unwrap()] = Empty;
            grid[*T_map.get_by_left(&(t - S)).unwrap()] = Empty;
            r -= 1;

            //Also remove from current matching
            let to_remove = M
                .iter()
                .position(|&(s_in_m, _t)| s_in_m == s)
                .expect("Soldier should be in mapping");
            M.remove(to_remove);

            continue;
        }

        //Now we need to find a cycle

        //Start at a soldier in H
        let soldier_in_h = H.edges().filter(|&(u, _v)| u <= S).next().unwrap().0;

        let mut cycle_edges = VecDeque::new();
        let mut edge = (soldier_in_h, H.adj_list(soldier_in_h).next().unwrap().1);
        let mut visited = BitVec::from_elem(H.num_v(), false);

        while !visited[edge.0] {
            visited.set(edge.0, true);
            cycle_edges.push_back(edge);
            debug!(
                "pushed Edge {:?} ",
                format!("{}->{}", vertex_to_string(edge.0), vertex_to_string(edge.1))
            );
            //adj list returns an (internal edge index, next vertex)
            edge = (edge.1, H.adj_list(edge.1).next().unwrap().1);
            debug!("Edge {:?} ", edge);
        }

        //cut to the actual cycle found
        let cycle_end = cycle_edges.back().unwrap().1;
        let cycle_start = cycle_edges
            .iter()
            .position(|&(u, _v)| u == cycle_end)
            .unwrap();
        cycle_edges.drain(0..cycle_start);

        debug!(
            "Cycle C =\n{}\n",
            cycle_edges
                .iter()
                .map(|&(u, v)| format!("{}->{}", vertex_to_string(u), vertex_to_string(v)))
                .collect::<Vec<_>>()
                .join("\n")
        );

        //Consider a new matching M' of G consisting of the edges of M whose reverse is not in C, p
        // lus the edges in C whose reverse is not in M. That is, M' is M but exchanging the edges
        // present in C in some direction. M' in this case is also a matching of G of the same size as M

        //because it is a cycle, we know we have new edges from G' to replace the ones removed from M
        let mut M_new: Vec<(usize, usize)> = Vec::new();
        M_new.extend(M.iter().filter(|&&(u, v)| !cycle_edges.contains(&(v, u))));
        M_new.extend(cycle_edges.iter().filter(|&&(u, v)| !M.contains(&(v, u))));

        debug!(
            "New matching M =\n{}\n",
            M_new
                .iter()
                .map(|&(u, v)| format!("{}->{}", vertex_to_string(u), vertex_to_string(v)))
                .collect::<Vec<_>>()
                .join("\n")
        );

        //Find all edges from G' which are actions we can take
        let st_actions = M_new
            .iter()
            .filter(|&uv| Gprime.contains(uv))
            .collect::<Vec<_>>();
        for &&(s, t) in st_actions.iter() {
            debug!("Taking actions from g' s {} t {}", s + 1, t + 1 - S);
            ans += &format!("{} {}\n", s + 1, t - S + 1);

            grid[*S_map.get_by_left(&s).unwrap()] = Empty;
            grid[*T_map.get_by_left(&(t - S)).unwrap()] = Empty;

            r -= 1;
        }

        M = M_new;
    }

    ans
}



fn build_graph(
    grid: &Grid<Tile>,
    is_g_prime: bool,
    M: usize,
    s_mapping: &BiMap<usize, GridCoord>,
    t_mapping: &BiMap<usize, GridCoord>,
    turret_reachable_squares_list: &Vec<HashSet<GridRowColVec>>
) -> IndexSet<(usize, usize)>
{
    let mut G: IndexSet<(usize, usize)> = IndexSet::new();

    let turret_locations = grid.filter_by_val(&Turret).collect::<HashSet<_>>();



    /*

        for (turret_index, turret_squares) in turret_squares_list.iter().enumerate() {
            debug!("Turret {} can see {:?}", turret_index, turret_squares);
        }
    */
    let soldier_locations = grid.filter_by_val(&Soldier).collect::<Vec<_>>();

    let S = soldier_locations.len();
    let T = turret_reachable_squares_list.len();

    for (_soldier_index, soldier_loc) in soldier_locations.iter().enumerate() {
        //debug!("BFS search on soldier {} @ {}", soldier_index, soldier_loc);

        //Node is location, distance, seen_turret
        let mut queue: VecDeque<(GridRowColVec, usize, bool)> = VecDeque::new();
        let mut visited = BitVec::from_elem(grid.C * grid.R, false);

        queue.push_back((soldier_loc.convert(), 0, false));
        visited.set(soldier_loc.0 * grid.C + soldier_loc.1, true);

        while !queue.is_empty() {
            let (loc, dist, seen_turret) = queue.pop_front().unwrap();

            let visible_turrets = turret_reachable_squares_list
                .iter()
                .enumerate()
                .filter(|(turret_index, turret_squares)|
                    turret_locations.contains(t_mapping.get_by_left(turret_index).unwrap() ) &&
                    turret_squares.contains(&loc))
                .map(|(turret_index, _)| turret_index);

            let mut turret_visible = false;
            for turret_index in visible_turrets {
                turret_visible = true;
                if !is_g_prime || (!seen_turret && is_g_prime) {
                    let s_vertex = *s_mapping.get_by_right(soldier_loc).unwrap();

                    //The turret index is already using the original grids index

                    /*debug!("Found s{} t{} mapped to soldier {} => {} at loc {}",
                    soldier_index, turret_index, s_vertex, t_vertex, loc);*/
                    G.insert((s_vertex, s_mapping.len() + turret_index));
                }
            }

            //no need to queue once we have been shot by a turret
            if is_g_prime && turret_visible {
                continue;
            }
            /*
            debug!(
                "Viewing {} dist {} seen turret? {} turret visible? {}",
                loc, dist, seen_turret, turret_visible
            );*/

            for dir in DIRECTIONS.iter() {
                let new_loc = loc + *dir;

                if let Some(tile) = grid.get_value(new_loc) {
                    if *tile == Building {
                        continue;
                    }

                    let newLocIndex = (new_loc.0 * grid.C as i64 + new_loc.1) as usize;
                    if visited[newLocIndex] {
                        continue;
                    }
                    visited.set(newLocIndex, true);
                    let new_dist = dist + 1;

                    if new_dist > M {
                        continue;
                    }

                    let new_seen_turret = seen_turret || turret_visible;

                    queue.push_back((new_loc, new_dist, new_seen_turret));
                }
            }
        }
    }

    debug!("Built graph from\n{}\n  S={} T={}", grid, S, T);
    G
}

impl Display for Grid<Tile>
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        for r in 0..self.R {
            for c in 0..self.C {
                if let Err(err) = write!(f, "{}", self[(r, c)]) {
                    return Err(err);
                }
            }
            if let Err(err) = writeln!(f, "") {
                return Err(err);
            }
        }
        write!(f, "")
    }
}
