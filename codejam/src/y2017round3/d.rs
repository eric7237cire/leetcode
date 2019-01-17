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

///1 + 2^2 + 3^3 + ...
fn sum_sq_closed_range(stop: usize) -> usize
{
    stop * (stop+1) * (2*stop+1)/ 6
}

fn calc_sum(inf_value:usize, width: usize, height: usize, D: usize, modulo: usize) -> usize
{
    let top_row_sum = D * sum_closed_range(width-1) + inf_value * width;
    //each row adds D*width more to each cell
    let square_sum = height * top_row_sum + D * width * sum_closed_range(height-1);

    println!("Calculated sum top left: {} width: {} height: {} D: {} == {}", inf_value, width, height, D,square_sum);
    square_sum
}


///assume top/left and bottom/right
fn calc_sum_2_influencers(top_left_inf_value:usize, bottom_right_inf_value:usize, width: usize, height: usize, D: usize, modulo: usize) -> usize
{
    /* we need to find the manhatten distance where they are equal.
      this will be distance from the top/left
      In the grid below, the manhattan distance is the distance to diagonal.

     This formula is derived by

     TL + MD = BR + (H + W - 2 - M) * D

     where
     TL -> value of top left node
     BR -> value of bottom right
     M -> manhattan distance (row + col diffs) starting from left corner
     D -> max change per cell
     W -> grid width
     H -> grid height

     Since we want them to meet, we need M steps from the top left, and H+W-2-M steps from the bottom right
     */
    let (top_left_manhat_dist, rem)  = div_rem(D * (width+height-2) + bottom_right_inf_value - top_left_inf_value, 2*D);
    println!("Manhatten Distance is {} {}", top_left_manhat_dist, rem);

    //the sphere of influence starting from the bottom right corner, -1 since it doesn't overlap with TL's influence
    let br_manhat_distance = height+width-2-top_left_manhat_dist-1;

    let mut top_rect_height = 0;
    let mut bot_rect_height = 0;
    let mut left_rect_width = 0;
    let mut right_rect_width = 0;
    let mut total_sum = 0;
    let mut triangle_start_row = 0;

    /*
         C0 |   C1 |   C2 |   C3 |   C4 |   C5 |   C6 |   C7 |   C8 |   C9 |
     +------+------+------+------+------+------+------+------+------+------+
  R0 | (TL) |   15 |   25 |   35 |   B  |   55 |   65 |   75 |   77 |   A  |
     +------+------+------+------+------+------+------+------+------+------+
  R1 |   15 |   25 |   35 |    B |   55 |   65 |   75 |   77 |    A | Arev |
     +------+------+------+------+------+------+------+------+------+------+
  R2 |   25 |   35 |   B  |   55 |   65 |   75 |   77 |    A | Arev |   47 |
     +------+------+------+------+------+------+------+------+------+------+
  R3 |   35 |   B  |   55 |   65 |   75 |   77 |    A | Arev |   47 |   37 |
     +------+------+------+------+------+------+------+------+------+------+
  R4 |    B |   15 |   25 |   35 |   45 |   A  | Arev |   75 |   77 |   C  |
     +------+------+------+------+------+------+------+------+------+------+
  R5 |   15 |   25 |   35 |   45 |    A | Arev |   75 |   77 |   C  |   57 |
     +------+------+------+------+------+------+------+------+------+------+
  R6 |   25 |   35 |   45 |   A  | Arev |   75 |   77 |    C |   57 |   47 |
     +------+------+------+------+------+------+------+------+------+------+
  R7 |   35 |   45 |    A | Arev |   75 |   77 |    C |   57 |   47 | (BR) |
     +------+------+------+------+------+------+------+------+------+------+

*/

    //Do we have a top rectangle (in ex. diag C would be [R0..R3]
    if top_left_manhat_dist >= width {
        top_rect_height = top_left_manhat_dist-width+1;
        total_sum = calc_sum(top_left_inf_value, width, top_rect_height, D, modulo);

    }

    //Do we have a bottom rectangle (in ex, diag B would be [R5..R7]
    if top_left_manhat_dist < height {
        bot_rect_height = top_left_manhat_dist - height - 1;
        total_sum += calc_sum(bottom_right_inf_value + (bot_rect_height+width-2 * D), width, bot_rect_height, D, modulo);
    }

    //Either we have a left or right rectangle

    //Left retangle (in ex, diag C would be [C0..C5]
    if top_left_manhat_dist >= height {
        left_rect_width = top_left_manhat_dist - height + 1;
        total_sum += calc_sum(top_left_inf_value, left_rect_width, height - top_rect_height - bot_rect_height, D, modulo);
    }

    //Right rectangle (in ex, diag B would be [C5..C9]
    if br_manhat_distance >= height {
        //one col is for the reverse of the triangle (see A-rev)
        //(width - 1) - (M - 1)
        right_rect_width = br_manhat_distance - height + 1;
        //stop 1 row short of BR's diagonal
        total_sum += calc_sum(bottom_right_inf_value, right_rect_width, height - top_rect_height - bot_rect_height, D, modulo);
    }

    //Now the actual diagonal/triangle
    let top_left_triangle_height = height - bot_rect_height - top_rect_height;
    //-1 is to take into account we have 2 columns in the jagged part
    let top_left_triangle_width = width - left_rect_width - right_rect_width - 1;
    assert_eq!(top_left_triangle_width, top_left_triangle_height);

    let top_left_triangle_seed = top_left_inf_value + (left_rect_width+top_rect_height) * D;
    let top_left_triangle_sum = sum_sq_closed_range(top_left_triangle_height) * D + sum_closed_range(top_left_triangle_height) * top_left_triangle_seed;
    total_sum += top_left_triangle_sum;

    
    let bottom_right_triangle_height = min(br_manhat_distance+1, height);
    let bottom_right_triangle_width = min(br_manhat_distance+1-right_rect_width, width-right_rect_width);
    assert_eq!(bottom_right_triangle_height, bottom_right_triangle_width);
    let bottom_right_triangle_seed = bottom_right_inf_value + (right_rect_width*D) + (bot_rect_height*D);
    let bottom_right_triangle_sum = sum_sq_closed_range(bottom_right_triangle_height) * D + sum_closed_range(bottom_right_triangle_height) * bottom_right_triangle_seed;;
    total_sum += bottom_right_triangle_sum;

    
    total_sum
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
//#[cfg(test)]
pub mod test_round3_d
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

    fn find_grid_sum_naive_two_influencer(
        corners: [usize;2],
        width: usize,
        height: usize,
        D: usize,
        modulo: usize,
    ) -> usize
    {
        let mut g: Grid<usize> = Grid::new( height, width);

        let corner_coords = vec![ IntCoord2d(0,0), IntCoord2d(height-1,width-1)];

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

    //#[test]
    pub fn test_grid_sum()
    {

        println!("Starting...");
        let mut rng: StdRng = SeedableRng::seed_from_u64(42);

        let grid_values = Uniform::from(3..50usize);
        let grid_dims = Uniform::from(3..20usize);
        for _ in 0..5 {
            let D = 10; //grid_values.sample(&mut rng);
            let mut corner_values:Vec<_> = (0..4).map(|_| grid_values.sample(&mut rng)).collect();
            let mut grid_width = grid_dims.sample(&mut rng);
            let mut grid_height = grid_dims.sample(&mut rng);

            grid_width = 10;
            grid_height = 7;
            corner_values[2] = 12;
            corner_values[1] = 65;

            let sum1 = find_grid_sum_naive_single_influencer(7, grid_width,grid_height, D, usize::MAX);
            let sum2 = calc_sum(7, grid_width,grid_height, D, usize::MAX);
            //let sum1 = find_grid_sum_naive(&corner_values, grid_width,grid_height, D, usize::MAX);
            //let sum2 = find_grid_sum(&corner_values, grid_width,grid_height, D, usize::MAX);

            println!("Sum1 {} Sum2 {}", sum1, sum2);
            assert_eq!(sum1, sum2);

            let sum3 = find_grid_sum_naive_two_influencer([5, 7], grid_width,grid_height, D, usize::MAX);
            let sum4 = calc_sum_2_influencers(5,7,grid_width,grid_height, D, usize::MAX);

            println!("Sum3 {} Sum4 {}", sum3, sum4);
            assert_eq!(sum3, sum4);
        }



    }

}
