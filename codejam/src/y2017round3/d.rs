use super::super::util::input::*;

use crate::algo::graph::disjointset::DisjointSet;
use std::cmp::min;
use std::io::Write;
use std::time::Instant;
use num_integer::div_rem;
use crate::util::grid::GridCoord;
use crate::util::grid::IntCoord2d;
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


fn find_intersection( start: usize, stop: usize, length: usize, D: usize) -> (usize, usize) {
    let (inter , rem) = div_rem(( length  * D  - D +stop  - start  ) , 2*D );
    let start_inf =  inter as usize;
    let stop_info = inter + if rem > 0 {1 } else {0};

    (start_inf, stop_info)
}

///Sum 0 to stop
fn sum_closed_range(stop: usize) -> usize
{
    stop * (stop+1) / 2
}

fn calc_sum(inf_value:usize, width: usize, height: usize, D: usize, modulo: usize) -> usize
{
    let top_row_sum = D * sum_closed_range(width-1) + inf_value * width;
    //each row adds D*width more to each cell
    let square_sum = height * top_row_sum + D * width * sum_closed_range(height-1);

    square_sum
}

///
/// Top left, top right, bottom right, bottom left
fn find_grid_sum(corners: &[usize], width: usize, height: usize, D: usize, modulo: usize) -> usize
{
    //Find extent of influence of top/left corner
    let (top_left_col, top_right_col) = find_intersection(corners[0], corners[1], width, D);
    let (left_top_row, left_bottom_row) = find_intersection(corners[0], corners[3], height, D);
    let (right_top_row, right_bottom_row) = find_intersection(corners[1], corners[2], height, D);
    let (bottom_left_col, bottom_right_col) = find_intersection(corners[3], corners[2], width, D);

    println!("Calculated top mid as {} {} ", top_left_col, top_right_col);
    println!("Calculated left top/bottom rows {} {} ", left_top_row, left_bottom_row);
    println!("Calculated right top/bottom rows {} {} ", right_top_row, right_bottom_row);
    println!("Calculated bottom l/r as cols {} {} ", bottom_left_col, bottom_right_col);

    let mut row_cut_offs = vec![left_top_row, left_bottom_row, right_top_row, right_bottom_row];
    let mut col_cut_offs = vec![ top_left_col, top_right_col, bottom_left_col, bottom_right_col];
    row_cut_offs.sort();
    col_cut_offs.sort();

    let corner_coords = vec! [ IntCoord2d(0,0), IntCoord2d(0, width-1), IntCoord2d(height-1, width-1), IntCoord2d(height-1,0)];
    let corner_with_val:Vec<(GridCoord,usize)> = corner_coords.iter().zip(corners.iter()).map(|t| (*t.0, *t.1)) .collect();

    //top / left - bottom / right pairs
    //number 123, 456, 789
/*
    let ss1 = calc_sum(corner_with_val[0], IntCoord2d(0,0), IntCoord2d(row_cut_offs[1], col_cut_offs[1]));
    let ss2 = calc_sum( if left_bottom_row == row_cut_offs[2] { corner_with_val[0] } else {corner_with_val[3]},
    IntCoord2d(row_cut_offs[1], 0), IntCoord2d(row_cut_offs[2], col_cut_offs[1]));
    let ss3 = calc_sum(corner_with_val[3], IntCoord2d(row_cut_offs[3],0), IntCoord2d(height-1, col_cut_offs[1]));

    let ss4 = calc_sum(if top_right_col==col_cut_offs[2] { corner_with_val[0] } else {corner_with_val[2]},
    IntCoord2d(0,col_cut_offs[1]), IntCoord2d(row_cut_offs[1], col_cut_offs[2]));

    let ss6 = calc_sum(if bottom_right_col==col_cut_offs[2] { corner_with_val[0] } else { corner_with_val[2]}, IntCoord2d(0,col_cut_offs[2]), IntCoord2d(row_cut_offs[1], col_cut_offs[3]));

    let ss7 = calc_sum(corner_with_val[1], IntCoord2d(0,col_cut_offs[2]), IntCoord2d(row_cut_offs[1], col_cut_offs[3]));
    let ss8 = calc_sum( if right_bottom_row == row_cut_offs[2] { corner_with_val[1] } else {corner_with_val[2]},
    IntCoord2d(row_cut_offs[1], col_cut_offs[3]), IntCoord2d(row_cut_offs[2], col_cut_offs[3]));
    let ss9 = calc_sum(corner_with_val[2], IntCoord2d(row_cut_offs[3],0), IntCoord2d(height-1, col_cut_offs[1]));
*/

  //  ss1+ss2+ss3+ss4+ss6+ss7+ss8+ss9
    3
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
        let mut g: Grid<usize> = Grid::new( height, width);

        let corner_coords:Vec<_> = (0..=3).map(|i|
            IntCoord2d::<usize>(if
                      i < 2 { 0 } else { height - 1 },
                      if i == 0 || i == 3 { 0 } else { width - 1 },
            )).collect();

        for (coord, val) in corner_coords.iter().zip(corners.iter()) {
            g[*coord] = *val;
        }

        g.transform(|(coord, val)| {
            let max_values = corner_coords.iter().zip(corners.iter()).map( |(cc,val)| val + cc.distance(&coord) * D);
            *val = max_values.min().unwrap();

            //println!("Set val {} loc {}", val, coord);
        });

        println!("Grid\n{:#.6?}\n corners {:?}\nvalues {:?}\nD {:?}", g, corner_coords, corners, D);
        // for

        g.iter_loc().map(|lv| lv.1).sum()
    }

    ///
    /// Assume inf is in top/left corner.  each grid cell gets D added to it

    fn find_grid_sum_naive_single_influencer(
        inf_value: usize,
        width: usize,
        height: usize,
        D: usize,
        modulo: usize,
    ) -> usize
    {
        let mut g: Grid<usize> = Grid::new( height, width);

        let corner_coord=
            IntCoord2d(0,0);

        g[0] = inf_value;

        g.transform(|(coord, val)| {
            *val = inf_value + corner_coord.distance(&coord) * D;

            //println!("Set val {} loc {}", val, coord);
        });

        println!("Grid\n{:#.6?}\nD {:?}", g, D);
        // for

        g.iter_loc().map(|lv| lv.1).sum()
    }

    #[test]
    fn test_grid_sum()
    {

        let mut rng: StdRng = SeedableRng::seed_from_u64(42);

        let grid_values = Uniform::from(3..50usize);
        let grid_dims = Uniform::from(3..20usize);
        for _ in 0..5 {
            let D = 10; //grid_values.sample(&mut rng);
            let mut corner_values:Vec<_> = (0..4).map(|_| grid_values.sample(&mut rng)).collect();
            let mut grid_width = grid_dims.sample(&mut rng);
            let grid_height = grid_dims.sample(&mut rng);

            grid_width = 10;
            corner_values[2] = 12;
            corner_values[1] = 65;

            let sum1 = find_grid_sum_naive_single_influencer(7, grid_width,grid_height, D, usize::MAX);
            let sum2 = calc_sum(7, grid_width,grid_height, D, usize::MAX);
            //let sum1 = find_grid_sum_naive(&corner_values, grid_width,grid_height, D, usize::MAX);
            //let sum2 = find_grid_sum(&corner_values, grid_width,grid_height, D, usize::MAX);

            println!("Sum1 {} Sum2 {}", sum1, sum2);

            assert_eq!(sum1, sum2);
        }



    }

}
