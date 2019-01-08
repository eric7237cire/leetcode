/*
Round 3 2008

Round Qual 2012, hall of mirrors
*/
use super::super::util::input::*;
//use super::super::util::math::*;
use std::ops::{Index,IndexMut};
use std::default::Default;
use std::fmt::{Display,Formatter,Result};
//use std::cmp::max;

pub fn solve_all_cases()
{
    let mut reader = InputReader::new();
    let t = reader.read_int();

    for case in 1..=t {
        let (R, C) = reader.read_tuple_2::<usize, usize>();
        let mut grid:Grid<char> = Grid::new(R,C);
        for r in 0..R {
            let row = reader.read_chars(C );
            for (c,t) in row.iter().enumerate() {
                grid[(r,c)] = *t;
            }
        }

        print!("{}", solve(case, &mut grid));
    }
}


#[derive(Debug, Copy, Clone)]
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
'#' =>     Wall,
'/' =>     ForwardMirror,
'\\' =>     BackwardMirror,
'|' =>     VerticalBeam,
'-' =>     HorizonalBeam,
            _ => panic!("Character not recognized: {}", item),
        }
    }
}

struct Grid<T>
{
    data : Vec<T> ,
    pub R: usize,
    pub C: usize
}

impl <T> Grid<T> {
    pub fn new(r: usize, c: usize) -> Grid<T> where T: Default {
        let mut g = Grid {R:r, C:c, data:Vec::new()};
        for _ in 0..r*c {
            g.data.push(Default::default());
        }
        g
    }
}

//get a row
impl <T> Index<usize> for Grid<T>  {
    type Output = [T];

    fn index<'a>(&'a self, row_index: usize) -> &'a [T] {
        &self.data[row_index*self.C..(row_index+1*self.C)]
    }
}
//get a cell
impl <T> Index<(usize,usize)> for Grid<T> {
    type Output = T;

    fn index<'a>(&'a self, row_col_index: (usize, usize)) -> &'a T {
        &self.data[row_col_index.0*self.C+row_col_index.1]
    }
}
//set a cell
impl <T> IndexMut<(usize,usize)> for Grid<T> {
    fn index_mut<'a>(&'a mut self, row_col_index: (usize,usize)) -> &'a mut T {
        & mut self.data[row_col_index.0*self.C+row_col_index.1]
    }
}
impl <T> Display for Grid<T> where T: Display {
    fn fmt(&self, f: &mut Formatter) -> Result {
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


fn solve(case_no: u32, grid: &mut Grid<char>) -> String
{
    debug!("Solving case {}", case_no);

    format!("Case #{}:\n{}\n", case_no, grid)
}
