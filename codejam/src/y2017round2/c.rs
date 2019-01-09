/*
Round 3 2008

Round Qual 2012, hall of mirrors
*/
use super::super::util::grid::constants::*;
use super::super::util::grid::{Grid, GridCoord, GridRowColVec, IntCoord2d};

use super::super::util::input::*;
use std::default::Default;
use std::fmt::{Display, Formatter};
use std::fmt;

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

    for i in 0..grid.R * grid.C {
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
                    return Err(r);
                } //   \\\\
                /*  => {
                    direction = match direction {

                        SOUTH => EAST,
                        EAST => SOUTH,
                        NORTH => WEST,
                        WEST => NORTH,
                    };
                }*/
                _ => {}
            };

            location += direction;
        } else {
            break;
        }
    }

    return Ok(r);
}

struct LaserChoice
{
    laser_index: usize,
    orientation: Tile,
}

type Trace = Vec<IntCoord2d<i16>>;
type OptionTrace = Option<Trace> ;

fn solve<'a>(case_no: u32, grid: &mut Grid<Tile>) -> String
{
    debug!("Solving case {}", case_no);

    let lasers = grid
        .filter_by_pred(|v| *v == VerticalBeam || *v == HorizonalBeam)
        .collect::<Vec<_>>();

    let laser_traces: Vec<[OptionTrace; 2]> = lasers
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

    let empty_squares = grid.filter_by_val(&Empty).collect::<Vec<_>>();

    let square_choices: Vec<Vec<LaserChoice>> = Vec::new();

    for (idx, es) in empty_squares.iter().enumerate() {
        for laser_data in laser_traces.iter() {

        }
    }

    for (laser_index, laser_loc) in lasers.iter().enumerate() {
        let traces = &laser_traces[laser_index];
        debug!(
            " Laser: {:?}\ntrace north/south {:?}\ntrace east/west {:?}\n",
            laser_loc, traces[0], traces[1],
        );
    }

    debug!(
        "Empties {:?} for \n{}",
        grid.filter_by_val(&Empty).take(2).collect::<Vec<_>>(),
        grid
    );
    format!("Case #{}:\n{}", case_no, grid)
}
