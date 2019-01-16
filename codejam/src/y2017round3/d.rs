use super::super::util::input::*;

use crate::algo::graph::disjointset::DisjointSet;
use std::cmp::min;
use std::io::Write;
use std::time::Instant;
//use std::cmp::max;

/*

*/
pub fn solve_all_cases()
{
    let now = Instant::now();

    let mut reader = InputReader::new();
    let t = reader.read_int();

    for case in 1..=t {
        let C = reader.read_int();

        print!("{}", solve(case, C,));
    }

    let duration = now.elapsed();
    let secs = f64::from(duration.as_secs() as u32) + f64::from(duration.subsec_nanos()) / 1e9f64;
    let _ = writeln!(
        ::std::io::stderr(),
        "\nElapsed time {:.2} second(s)\n",
        secs
    );
}
fn solve(case_no: u32, C: usize) -> String
{
    debug!("\n\n\nSolving case {}", case_no);

    format!("Case #{}: \n", case_no)
}

///
/// Top left, top right, bottom right, bottom left
fn find_grid_sum(corners: &[usize], width: usize, height: usize, D: usize, modulo: usize) -> usize
{
    8
}

//cargo test round3_d -- --nocapture
#[cfg(test)]
mod test_round3_d
{
    use super::*;
    use crate::util::grid::Grid;
    use crate::util::grid::GridCoord;
    use crate::util::grid::*;
    //use rand::distributions::{ Range};
    use rand::{Rng, StdRng, SeedableRng};
    use rand::distributions::{Distribution, Uniform};
    //use rand::rand_core::SeedableRng;
    use std::usize;

    fn find_grid_sum_naive(
        corners: &[usize],
        width: usize,
        height: usize,
        D: usize,
        modulo: usize,
    ) -> usize
    {
        let g: Grid<usize> = Grid::new(width, height);

        let corner_coords = (0..=3).map(|i|
            IntCoord2d::<usize>(if
                      i < 2 { 0 } else { height - 1 },
                      if i == 0 || i == 3 { 0 } else { width - 1 },
            ));

        println!("Grid\n{:#.6?}\n corners {:?}", g, corner_coords);
        // for
        8
    }

    #[test]
    fn test_grid_sum()
    {

        let mut rng: StdRng = SeedableRng::seed_from_u64(42);

        let grid_values = Uniform::from(3..50usize);
        let grid_dims = Uniform::from(3..20usize);
        for _ in 0..5 {
            let D = grid_values.sample(&mut rng);
            let corner_values:Vec<_> = (0..4).map(|_| grid_values.sample(&mut rng)).collect();
            let grid_width = grid_dims.sample(&mut rng);
            let grid_height = grid_dims.sample(&mut rng);

            let sum1 = find_grid_sum_naive(&corner_values, grid_width,grid_height, D, usize::MAX);
            let sum2 = find_grid_sum(&corner_values, grid_width,grid_height, D, usize::MAX);

            assert_eq!(sum1, sum2);
        }



    }

}
