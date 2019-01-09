/*
Round 3 2008

Round Qual 2012, hall of mirrors
*/
use super::super::util::grid::{Grid, GridConsts, GridRowCol, GridRowColVec};
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
fn trace_ray(grid: &Grid<Tile>, location: GridRowCol, direction: GridRowColVec) -> Vec<GridRowCol>
{
    let mut location = location + direction;
    let mut direction = direction;
    let mut r: Vec<GridRowCol> = Vec::new();

    for _ in 0..grid.R * grid.C {
        if let Some(tile) = grid.get_value(location) {
            match *tile {
                Wall => {
                    break;
                }
                Empty => {}
                //  /
                ForwardMirror | BackwardMirror => {
                    let mul = if *tile == ForwardMirror { 1 } else { -1 };
                    direction = match direction {
                        GridConsts::NORTH => GridConsts::EAST * mul,
                        GridConsts::EAST => GridConsts::NORTH * mul,
                        GridConsts::SOUTH => GridConsts::WEST * mul,
                        GridConsts::WEST => GridConsts::SOUTH * mul,
                        _ => direction,
                    };
                }
                VerticalBeam | HorizonalBeam => {
                    break;
                } //   \\\\
                  /*  => {
                      direction = match direction {

                          GridConsts::SOUTH => GridConsts::EAST,
                          GridConsts::EAST => GridConsts::SOUTH,
                          GridConsts::NORTH => GridConsts::WEST,
                          GridConsts::WEST => GridConsts::NORTH,
                      };
                  }*/
            };
            location += direction;
            r.push(location);
        } else {
            break;
        }
    }

    return r;
}

fn solve(case_no: u32, grid: &mut Grid<Tile>) -> String
{
    debug!("Solving case {}", case_no);
    debug!(
        "Empties {:?} for \n{}",
        grid.filter_byval(&Empty).collect::<Vec<_>>(),
        grid
    );
    format!("Case #{}:\n{}", case_no, grid)
}
