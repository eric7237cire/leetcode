/*
Round 3 2008

Round Qual 2012, hall of mirrors
*/
use super::super::util::grid::constants::*;
use super::super::util::grid::{Grid, GridCoord, GridRowColVec, IntCoord2d};

use super::super::util::input::*;
//use super::super::util::math::*;
//use std::ops::{Index,IndexMut};
use std::default::Default;
use std::fmt::{Display, Formatter, Result};
//use std::iter;
//use std::cmp::max;

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
    fn fmt(&self, f: &mut Formatter) -> Result
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
) -> (Vec<IntCoord2d<i16>>, bool)
{
    let mut location: IntCoord2d<i16> = location.convert();
    let mut direction = direction;
    let mut r: Vec<_> = Vec::new();
    let mut ok = true;

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
                    ok = false;
                    break;
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

    return (r, ok);
}

fn solve(case_no: u32, grid: &mut Grid<Tile>) -> String
{
    debug!("Solving case {}", case_no);

    let lasers = grid
        .filter_by_pred(|v| *v == VerticalBeam || *v == HorizonalBeam)
        .collect::<Vec<_>>();

    let laser_traces: Vec<Vec<Option<Vec<IntCoord2d<i16>>>>> = lasers
        .iter()
        .map(|loc| {
            let mut traces = Vec::new();
            for &dir in DIRECTIONS.iter() {
                let trace_ok = trace_ray(grid, *loc, dir);
                traces.push(if trace_ok.1 { Some(trace_ok.0) } else { None });
            }
            traces
        })
        .collect();

    for (laser_index, laser_loc) in lasers.iter().enumerate() {
        let traces = &laser_traces[laser_index];
        debug!(
            " Laser: {:?}\ntrace north {:?}\ntrace east {:?}\ntrace south {:?}\ntrace west {:?}\n",
            laser_loc, traces[0], traces[1], traces[2], traces[3],
        );
    }

    debug!(
        "Empties {:?} for \n{}",
        grid.filter_byval(&Empty).take(2).collect::<Vec<_>>(),
        grid
    );
    format!("Case #{}:\n{}", case_no, grid)
}
