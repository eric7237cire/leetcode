/*
Round 3 2008

like Round Qual 2012, hall of mirrors

Simulation, grid, backtracking

2-satisfiability / 2SAT

*/
use super::super::algo_ebtech::graph::connectivity::ConnectivityGraph;
use super::super::algo_ebtech::graph::Graph;
use super::super::util::grid::constants::*;
use super::super::util::grid::{Grid, GridCoord, GridRowColVec, IntCoord2d};
use super::super::util::input::*;
use bimap::BiMap;
use std::default::Default;
use std::fmt;
use std::fmt::{Display, Formatter};

pub fn solve_all_cases()
{
    let mut reader = InputReader::new();
    let t = reader.read_int();

    for case in 1..=t {
        let (R, C) = reader.read_tuple_2::<usize, usize>();
        let mut grid: Grid<Tile> = Grid::new(R, C);
        for r in 0..R {
            let row = reader.read_chars(C);
            for (c, t) in row.iter().enumerate() {
                grid[(r, c)] = Tile::from(*t);
            }
        }

        //if case != 31 {continue;}
        print!("{}", solve(case, &mut grid));
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tile
{
    Empty,
    Wall,
    ForwardMirror,
    BackwardMirror,
    VerticalBeam,
    HorizonalBeam,
}

use self::Tile::*;

impl Tile
{
    fn to_char(self) -> char
    {
        match self {
            Empty => '.',
            Wall => '#',
            ForwardMirror => '/',
            BackwardMirror => '\\',
            VerticalBeam => '|',
            HorizonalBeam => '-',
        }
    }
}

impl From<char> for Tile
{
    fn from(item: char) -> Self
    {
        match item {
            '.' => Empty,
            '#' => Wall,
            '/' => ForwardMirror,
            '\\' => BackwardMirror,
            '|' => VerticalBeam,
            '-' => HorizonalBeam,
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
fn trace_ray(
    grid: &Grid<Tile>,
    location: GridCoord,
    direction: GridRowColVec,
) -> Result<Vec<IntCoord2d<i16>>, Vec<IntCoord2d<i16>>>
{
    let mut location: IntCoord2d<i16> = location.convert();
    let mut direction = direction;
    let mut r: Vec<_> = Vec::new();
    //debug!("\nTracing {} starting at {}", location, direction);

    for i in 0..=grid.R * grid.C * grid.R * grid.C {
        if let Some(tile) = grid.get_value(location) {
            match *tile {
                Wall => {
                    break;
                }
                Empty => {
                    r.push(location);
                }

                ForwardMirror | BackwardMirror => {
                    let mul = if *tile == ForwardMirror { 1 } else { -1 };
                    direction = match direction {
                        NORTH => EAST * mul,
                        EAST => NORTH * mul,
                        SOUTH => WEST * mul,
                        WEST => SOUTH * mul,
                        _ => direction,
                    };
                }
                VerticalBeam | HorizonalBeam if i > 0 => {
                    r.push(location);
                    debug!("Err beam");
                    return Err(r);
                }
                _ => {}
            };

            location += direction;
        } else {
            break;
        }
    }

    return Ok(r);
}

#[derive(Debug)]
struct LaserChoice
{
    laser_index: usize,
    orientation: Tile,
}

//to use algo, Vertical is even, Horizonal is odd, so scheme is 2*laser_index + 1 if horiz.
fn get_graph_vertex_index(lc: &LaserChoice) -> usize
{
    2 * lc.laser_index + if lc.orientation == VerticalBeam { 0 } else { 1 }
}

type Trace = Vec<IntCoord2d<i16>>;
type OptionTrace = Option<Trace>;

fn solve<'a>(case_no: u32, grid: &mut Grid<Tile>) -> String
{
    debug!("Solving case {}", case_no);

    let laser_coords = grid
        .filter_by_pred(|v| *v == VerticalBeam || *v == HorizonalBeam)
        .collect::<Vec<_>>();

    //build up the list of coords a laser hits vert and horizontally
    let laser_traces: Vec<[OptionTrace; 2]> = laser_coords
        .iter()
        .map(|loc| {
            let mut combined_traces: [OptionTrace; 2] = [None, None];

            for (idx, &dir) in DIRECTIONS.iter().enumerate() {
                let trace_result = trace_ray(grid, *loc, dir);
                if let Ok(trace) = trace_result {
                    if idx < 2 {
                        combined_traces[idx] = Some(trace);
                    } else if combined_traces[idx - 2] != None {
                        if let Some(v) = combined_traces[idx % 2].as_mut() {
                            v.extend(trace);
                        }
                    }
                } else {
                    combined_traces[idx % 2] = None
                }
            }

            combined_traces
        })
        .collect();

    //list of coords of empty squares
    let empty_squares = grid.filter_by_val(&Empty).collect::<Vec<_>>();

    let mut square_choices: Vec<Vec<LaserChoice>> = Vec::new();
    let mut square_coords: BiMap<usize, IntCoord2d<usize>> = BiMap::new();

    for (empty_square_index, es) in empty_squares.iter().enumerate() {
        square_coords.insert(empty_square_index, *es);

        let mut sc = Vec::new();

        for (laser_index, laser_data) in laser_traces.iter().enumerate() {
            if laser_data[0] == None && laser_data[1] == None {
                return format!("Case #{}: IMPOSSIBLE\n", case_no);
            }
            for i in 0..2 {
                if let Some(ns) = &laser_data[i] {
                    if ns.contains(&es.convert()) {
                        sc.push(LaserChoice {
                            laser_index,
                            orientation: if i == 0 { VerticalBeam } else { HorizonalBeam },
                        });
                    }
                }
            }
        }

        //if a square can't get hit with any laser, we have no solution
        if sc.is_empty() {
            return format!("Case #{}: IMPOSSIBLE\n", case_no);
        }

        //more than 2 implies lasers should be hitting each other
        assert!(sc.len() <= 2);

        square_choices.push(sc);
    }

    //to use algo, must have even numbers, so vertex scheme is 2*laser_index if vertical
    //2*len + 2*laser_index if horizonal

    let mut graph = Graph::new(2 * laser_coords.len(), 8 * laser_coords.len());

    for (idx, sc) in square_choices.iter().enumerate() {
        if sc.len() == 1 {
            let vi = get_graph_vertex_index(&sc[0]);
            graph.add_two_sat_clause(vi, vi);
        } else {
            let v1 = get_graph_vertex_index(&sc[0]);
            let v2 = get_graph_vertex_index(&sc[1]);
            graph.add_two_sat_clause(v1, v2);
        }
        debug!("For square {} choices are {:?}", empty_squares[idx], sc);
    }
    for (laser_index, lt) in laser_traces.iter().enumerate() {
        for i in 0..2 {
            //invalid so we need  to tell the 2sat graph that
            if lt[i] == None {
                let v1 = get_graph_vertex_index(&LaserChoice {
                    laser_index,
                    orientation: if i == 0 { VerticalBeam } else { HorizonalBeam },
                });
                graph.add_two_sat_clause(v1 ^ 1, v1 ^ 1);
            }
        }
    }

    if true {
        let tsa = ConnectivityGraph::new(&graph, true).two_sat_assign();

        if tsa.is_none() {
            return format!("Case #{}: IMPOSSIBLE\n", case_no);
        } else {
            let tsa = tsa.unwrap();
            debug!("2SAT results {:?}.   laser_coords: {:?}", tsa, laser_coords);
            for (idx, &b) in tsa.iter().enumerate() {
                grid[laser_coords[idx]] = if b { VerticalBeam } else { HorizonalBeam };
            }
            format!("Case #{}: POSSIBLE\n{}", case_no, grid)
        }
    } else {
        //what I did, backtracking.  must be release mode

        let mut is_covered: Vec<i16> = vec![0; square_choices.len()];
        if !helper(
            grid,
            &laser_traces,
            &laser_coords,
            &square_coords,
            &square_choices,
            0,
            &mut is_covered,
        ) {
            return format!("Case #{}: IMPOSSIBLE\n", case_no);
        }

        format!("Case #{}: POSSIBLE\n{}", case_no, grid)
    }
}

fn helper(
    grid: &mut Grid<Tile>,
    laser_traces: &Vec<[OptionTrace; 2]>,
    laser_coords: &Vec<IntCoord2d<usize>>,
    square_coords: &BiMap<usize, IntCoord2d<usize>>,
    square_choices: &Vec<Vec<LaserChoice>>,
    current_laser_index: usize,
    is_covered: &mut Vec<i16>,
) -> bool
{
    //base case
    if current_laser_index == laser_traces.len() {
        return *is_covered.iter().min().unwrap() > 0;
    }

    //short circuit; check lasers less than index
    for lc in square_choices {
        if lc
            .iter()
            .filter(|&c| {
                c.laser_index >= current_laser_index
                    || c.orientation == grid[laser_coords[c.laser_index]]
            })
            .count()
            <= 0
        {
            return false;
        }
    }

    let laser_data = &laser_traces[current_laser_index];
    //try vertical
    for ld_idx in 0..2 {
        if let Some(ns) = &laser_data[ld_idx] {
            grid[laser_coords[current_laser_index]] = if ld_idx == 0 {
                VerticalBeam
            } else {
                HorizonalBeam
            };

            for coord in ns {
                is_covered[*square_coords.get_by_right(&coord.convert()).unwrap()] += 1;
            }

            let ok = helper(
                grid,
                laser_traces,
                laser_coords,
                square_coords,
                square_choices,
                current_laser_index + 1,
                is_covered,
            );

            if ok {
                return true;
            } else {
                for coord in ns {
                    is_covered[*square_coords.get_by_right(&coord.convert()).unwrap()] -= 1;
                }
            }
        }
    }

    return false;
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
