// maybe like Problem D. Descending in the Dark round 2 2012

/*
Bipartite matching
Grid
BFS
Hard
*/
use super::super::algo::graph::flow::*;
//use super::super::algo::graph::*;
use super::super::util::grid::constants::*;
use super::super::util::grid::{Grid, GridCoord, GridRowColVec};
use super::super::util::input::*;
//use bimap::BiMap;
use std::default::Default;
use std::fmt;
use std::fmt::{Display, Formatter};

pub fn solve_all_cases()
{
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

        //if case != 31 {continue;}
        print!("{}", solve(case, &mut grid, M));
    }
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
use bit_vec::BitVec;
use std::collections::HashSet;
use std::collections::VecDeque;

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

fn solve<'a>(case_no: u32, grid: &mut Grid<Tile>, M: usize) -> String
{
    debug!("Solving case {}\nM={}\n{}\n", case_no, M, grid);

    let turret_squares_list = grid
        .filter_by_val(&Turret)
        .map(|t_loc| reachable(&grid, t_loc))
        .collect::<Vec<_>>();

    for (turret_index, turret_squares) in turret_squares_list.iter().enumerate() {
        debug!("Turret {} can see {:?}", turret_index, turret_squares);
    }

    let S = grid.filter_by_val(&Soldier).count();
    let T = grid.filter_by_val(&Turret).count();

    //Construct the 2 graphs: G and G'
    let mut G1 = FlowGraph::new(2 + S + T, 4);
    let mut G2 = FlowGraph::new(2 + S + T, 4);

    let source = S + T;
    let sink = S + T + 1;

    //BFS for each soldier

    //will be in left to right order, then top down order
    for (soldier_index, soldier_loc) in grid.filter_by_val(&Soldier).enumerate() {
        debug!("BFS search on soldier {} @ {}", soldier_index, soldier_loc);

        //Node is location, distance, seen_turret
        let mut queue: VecDeque<(GridRowColVec, usize, bool)> = VecDeque::new();
        let mut visited = BitVec::from_elem(grid.C * grid.R, false);

        queue.push_back((soldier_loc.convert(), 0, false));
        visited.set(soldier_loc.0 * grid.C + soldier_loc.1, true);

        while !queue.is_empty() {
            let (loc, dist, seen_turret) = queue.pop_front().unwrap();

            let visible_turrets = turret_squares_list
                .iter()
                .enumerate()
                .filter(|&(_turret_index, turret_squares)| turret_squares.contains(&loc))
                .map(|(turret_index, _)| turret_index);

            let mut turret_visible = false;
            for turret_index in visible_turrets {
                turret_visible = true;
                G1.add_edge(soldier_index, S + turret_index, 1, 1);

                if !seen_turret {
                    G2.add_edge(soldier_index, S + turret_index, 1, 1);
                }
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

    //Now find max matching of G1 (G has an edge from soldier s to turret t if and only if soldier s can destroy turret t after all other turrets have been destroyed)
    for s in 0..S {
        G1.add_edge(source, s, 1, 1);
    }

    for t in S..S + T {
        G1.add_edge(t, sink, 1, 1);
    }

    let (R, flow) = G1.dinic(source, sink);

    debug!(
        "U->V edges, flow in G:\n{}",
        flow.iter()
            .enumerate()
            .filter(|&(_e, f)| *f >= 0)
            //map to u->v
            .map(|(e, f)| (G1.graph.endp[e ^ 1], G1.graph.endp[e], f))
            .map(|(u, v, f)| format!(
                "{} => {}; {}",
                match u {
                    s if s < S => format!("Soldier {}", s + 1),
                    t if t >= S && t < S + T => format!("Turret #{} ({})", t - S + 1, t),
                    v if v == sink => "Sink".to_string(),
                    _source => "Source".to_string(),
                },
                match v {
                    s if s < S => format!("Soldier {}", s + 1),
                    t if t >= S && t < S + T => format!("Turret {} ({})", t - S + 1, t),
                    v if v == sink => "Sink".to_string(),
                    _source => "Source".to_string(),
                },
                format!("Flow: {}", f)
            ))
            .collect::<Vec<String>>()
            .join("\n")
    );

    debug!(
        "U->V edges in maximum matching:\n{:?}",
        flow.iter()
            .enumerate()
            .filter(|&(_e, f)| *f > 0)
            //map to u->v
            .map(|(e, _f)| (G1.graph.endp[e ^ 1], G1.graph.endp[e]))
            //leave out source and sink nodes
            .filter(|&(u, v)| u != source && v != sink)
            .collect::<Vec<_>>()
    );

    format!("Case #{}: POSSIBLE\n{}", case_no, grid)
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
