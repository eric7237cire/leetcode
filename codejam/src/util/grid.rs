use std::default::Default;
use std::fmt::{Display, Formatter, Result};
use std::ops::{AddAssign,Add,Mul};
use std::ops::{Index, IndexMut};
//use num::{Integer,cast,NumCast};

pub struct Grid<T>
{
    data: Vec<T>,
    pub R: usize,
    pub C: usize,
}

#[derive(Debug, Copy, Clone)]
pub struct GridRowCol(usize, usize);

#[derive(Debug, Copy, Clone,PartialEq, Eq)]
pub struct GridRowColVec(i64, i64);

pub struct GridConsts {}

impl GridConsts
{
    pub const NORTH: GridRowColVec = GridRowColVec(-1, 0);
    pub const EAST: GridRowColVec = GridRowColVec(0, 1);
    pub const SOUTH: GridRowColVec = GridRowColVec(1, 0);
    pub const WEST: GridRowColVec = GridRowColVec(0, -1);

    pub const DIRECTIONS: [GridRowColVec; 4] = [
        GridConsts::NORTH,
        GridConsts::EAST,
        GridConsts::SOUTH,
        GridConsts::WEST,
    ];
}

impl<T> Grid<T>
{
    pub fn new(r: usize, c: usize) -> Grid<T>
    where
        T: Default,
    {
        let mut g = Grid {
            R: r,
            C: c,
            data: Vec::new(),
        };
        for _ in 0..r * c {
            g.data.push(Default::default());
        }
        g
    }

    pub fn get_value<'a>(&'a self, row_col_index: GridRowCol) -> Option<&'a T>
    {
        if  row_col_index.0 >= self.R
            || row_col_index.1 >= self.C
        {
            None
        } else {
            Some(&self.data[row_col_index.0 * self.C + row_col_index.1])
        }

    }
}

//get a row
impl<T> Index<usize> for Grid<T>
{
    type Output = [T];

    fn index<'a>(&'a self, row_index: usize) -> &'a [T]
    {
        &self.data[row_index * self.C..(row_index + 1 * self.C)]
    }
}
//get a cell
impl<T> Index<GridRowCol> for Grid<T>
{
    type Output = T;

    fn index<'a>(&'a self, row_col_index: GridRowCol) -> &'a T
    {
        if row_col_index.0 >= self.R
            || row_col_index.1 >= self.C
        {
            panic!("RowCol {:?} invalid for grid {}, {}",row_col_index, self.R,self.C);
        }

        &self.data[row_col_index.0 * self.C + row_col_index.1]

    }
}
//set a cell
impl<T> IndexMut<GridRowCol> for Grid<T>
{
    fn index_mut<'a>(&'a mut self, row_col_index: GridRowCol) -> &'a mut T
    {
        &mut self.data[row_col_index.0 * self.C + row_col_index.1]
    }
}
impl<T> Index<(usize, usize)> for Grid<T>
{
    type Output = T;

    fn index<'a>(&'a self, row_col_index: (usize, usize)) -> &'a T
    {
        &self.data[row_col_index.0 * self.C + row_col_index.1]
    }
}
//set a cell
impl<T> IndexMut<(usize, usize)> for Grid<T>
{
    fn index_mut<'a>(&'a mut self, row_col_index: (usize, usize)) -> &'a mut T
    {
        &mut self.data[row_col_index.0 * self.C + row_col_index.1]
    }
}
impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> Result
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

impl Add<GridRowColVec> for GridRowCol
{
    type Output = GridRowCol;

    fn add(self, other: GridRowColVec) -> GridRowCol
    {
        GridRowCol(
            ((self.0 as i64) + other.0) as usize,
            ((self.1 as i64) + other.1) as usize,
        )
    }
}

impl Add<GridRowCol> for GridRowCol
{
    type Output = Self;

    fn add(self, other: Self) -> Self
    {
        GridRowCol(self.0 + other.0, self.1 + other.1)
    }
}
impl AddAssign<GridRowColVec> for GridRowCol {
    fn add_assign(&mut self, other: GridRowColVec) {
        *self = *self + other
    }
}

impl Mul<i32> for GridRowColVec  {

    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        GridRowColVec(self.0 * rhs as i64, self.1 * rhs as i64)
    }
}