use num::{cast, Integer, NumCast};
use std::cmp::PartialEq;
use std::default::Default;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::ops::{Add, AddAssign, Mul};
use std::ops::{Index, IndexMut};

pub struct Grid<T>
{
    data: Vec<T>,
    pub R: usize,
    pub C: usize,
}

pub trait GridCoordTrait: Hash + Integer + Display + NumCast + Copy + Mul + Add
{
}

impl<N> GridCoordTrait for N where N: Hash + Integer + Display + NumCast + Copy + Mul + Add {}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct IntCoord2d<T>(pub T, pub T)
where
    T: GridCoordTrait;

pub type GridCoord = IntCoord2d<usize>;

/*
impl <F> From<IntCoord2d<F>> for GridCoord {
    fn from(coord: IntCoord2d<F>) -> Self {
        IntCoord2d::<usize>(cast::<F,_>(coord.0).unwrap(),
                            cast::<F,_>(coord.1).unwrap())
    }
}*/

impl From<IntCoord2d<i64>> for GridCoord
{
    fn from(coord: IntCoord2d<i64>) -> Self
    {
        IntCoord2d::<usize>(coord.0 as usize, coord.1 as usize)
    }
}
/*
impl From<IntCoord2d<usize>> for GridCoord {
    fn from(coord: IntCoord2d<usize>) -> Self {
        coord
    }
}*/

impl<N: GridCoordTrait> IntCoord2d<N>
{
    pub fn convert<M: GridCoordTrait>(&self) -> IntCoord2d<M>
    {
        IntCoord2d::<M>(cast::<N, M>(self.0).unwrap(), cast::<N, M>(self.1).unwrap())
    }
}

pub type GridRowColVec = IntCoord2d<i64>;

//pub struct GridConsts {}

pub mod constants
{
    use super::*;

    pub const NORTH: GridRowColVec = IntCoord2d(-1, 0);
    pub const EAST: GridRowColVec = IntCoord2d(0, 1);
    pub const SOUTH: GridRowColVec = IntCoord2d(1, 0);
    pub const WEST: GridRowColVec = IntCoord2d::<i64>(0, -1);

    pub const DIRECTIONS: [IntCoord2d<i64>; 4] = [NORTH, EAST, SOUTH, WEST];
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

    pub fn get_value<'a, N: GridCoordTrait>(&'a self, row_col_index: IntCoord2d<N>)
        -> Option<&'a T>
    {
        if row_col_index.0 < N::zero() || row_col_index.1 < N::zero() {
            return None;
        }
        let row_col_index: IntCoord2d<usize> = row_col_index.convert();

        if row_col_index.0 >= self.R || row_col_index.1 >= self.C {
            return None;
        }

        Some(&self.data[row_col_index.0 * self.C + row_col_index.1])
    }

    pub fn filter_by_val<'a>(&'a self, val: &'a T) -> impl Iterator<Item = GridCoord> + 'a
    where
        //I: 'a,
        T: PartialEq,
    {
        self.data
            .iter()
            .enumerate()
            .filter(move |(_index, value)| *value == val)
            .map(move |(index, _value)| IntCoord2d(index / self.C, index % self.C))
    }

    pub fn filter_by_pred<'a, P>(&'a self, predicate: P) -> impl Iterator<Item = GridCoord> + 'a
    where
        P: Fn(&T) -> bool + 'a,
        T: PartialEq,
    {
        self.data
            .iter()
            .enumerate()
            .filter(move |(_index, value)| predicate(*value))
            .map(move |(index, _value)| IntCoord2d(index / self.C, index % self.C))
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
impl<T, N: GridCoordTrait> Index<IntCoord2d<N>> for Grid<T>
{
    type Output = T;

    fn index<'a>(&'a self, row_col_index: IntCoord2d<N>) -> &'a T
    {
        let row_col_index: IntCoord2d<usize> = row_col_index.convert();
        if row_col_index.0 >= self.R || row_col_index.1 >= self.C {
            panic!(
                "RowCol {:?} invalid for grid {}, {}",
                row_col_index, self.R, self.C
            );
        }

        &self.data[row_col_index.0 * self.C + row_col_index.1]
    }
}
//set a cell
impl<T, N: GridCoordTrait> IndexMut<IntCoord2d<N>> for Grid<T>
{
    fn index_mut<'a>(&'a mut self, row_col_index: IntCoord2d<N>) -> &'a mut T
    {
        let row_col_index: IntCoord2d<usize> = row_col_index.convert();
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

impl<T> Debug for Grid<T>
where
    T: Display,
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

#[cfg(test)]
mod tests
{
    use self::super::constants::*;
    use self::super::*;

    #[test]
    fn test_add()
    {
        assert_eq!(IntCoord2d::<u8>(0, 2), IntCoord2d::<u8>(0, 3) + WEST);
    }

    #[test]
    fn test_get_value()
    {
        let mut grid: Grid<char> = Grid::new(2, 2);
        grid[(0, 0)] = 'a';
        grid[(1, 0)] = 'b';
        grid[(1, 1)] = 'd';

        assert_eq!(Some(&'d'), grid.get_value(IntCoord2d::<i16>(1, 1)));
    }
}

/// A + B will convert B to A's unit
impl<N: GridCoordTrait, M: GridCoordTrait> Add<IntCoord2d<M>> for IntCoord2d<N>
{
    type Output = Self;

    fn add(self, rhs: IntCoord2d<M>) -> Self
    {
        let lhs: IntCoord2d<M> = self.convert();

        IntCoord2d(
            cast::<M, N>(lhs.0 + rhs.0).unwrap(),
            cast::<M, N>(lhs.1 + rhs.1).unwrap(),
        )
    }
}
impl<N: GridCoordTrait> AddAssign<GridRowColVec> for IntCoord2d<N>
{
    fn add_assign(&mut self, other: GridRowColVec)
    {
        *self = *self + other
    }
}

impl<N: GridCoordTrait, M: GridCoordTrait> Mul<M> for IntCoord2d<N>
{
    type Output = Self;

    fn mul(self, rhs: M) -> Self
    {
        let rhs: N = cast::<M, N>(rhs).unwrap();
        IntCoord2d::<N>(self.0 * rhs, self.1 * rhs)
    }
}

impl<N: GridCoordTrait> Debug for IntCoord2d<N>
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        write!(f, "(R{}, C{})", self.0, self.1)
    }
}
impl<N: GridCoordTrait> Display for IntCoord2d<N>
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        write!(f, "(R{}, C{})", self.0, self.1)
    }
}
