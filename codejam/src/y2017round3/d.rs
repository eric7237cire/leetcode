use self::constants::*;
use crate::algo::large_nums::*;
use crate::util::codejam::run_cases;
use crate::util::grid::Grid;
use itertools::Itertools;
use num_integer::div_rem;
use std::cmp::min;
use std::collections::HashSet;
use std::io::Write;

/*
Grids
Modulo arithmetic
Large numbers
Math
Graph / shortest paths in grid
*/
pub fn solve_all_cases()
{
    run_cases(
        &["D-small-practice", "D-large-practice"],
        "y2017round3",
        |reader, buffer| {
            let t = reader.read_int();
            for case in 1..=t {
                let i1 = reader.read_num_line();
                let height = i1[0];
                let width = i1[1];
                let num_fixed = i1[2];
                let D = i1[3];

                let fixed_values: Vec<_> = (0..num_fixed)
                    .map(|_| reader.read_tuple_3::<usize>())
                    .map(|(r, c, v)| (r - 1, c - 1, v))
                    .collect();

                write!(
                    buffer,
                    "{}",
                    solve(case, width, height, D, &fixed_values[..])
                )
                .unwrap();
            }
        },
    );
}

fn abs_diff(a: usize, b: usize) -> usize
{
    (a as i64 - b as i64).abs() as usize
}

/// len of range [a,b] or a..=b a..b+1
fn inclusive_range_len(a: usize, b: usize) -> usize
{
    b - a + 1
}

fn solve(
    case_no: u32,
    width: usize,
    height: usize,
    D: usize,
    fixed_values: &[(usize, usize, usize)],
) -> String
{
    //println!("Solving case {}", case_no);

    for &(r1, c1, val1) in fixed_values.iter() {
        for &(r2, c2, val2) in fixed_values.iter() {
            if !is_valid(val1, val2, 1 + abs_diff(r1, r2) + abs_diff(c1, c2), D) {
                return format!("Case #{}: IMPOSSIBLE\n", case_no);
            }
        }
    }

    let mut interesting_rows: HashSet<usize> = [0, height - 1].iter().cloned().collect();
    let mut interesting_cols: HashSet<usize> = [0, width - 1].iter().cloned().collect();

    for &(row, col, _) in fixed_values.iter() {
        interesting_rows.insert(row);
        interesting_cols.insert(col);
    }

    let interesting_cols: Vec<usize> = interesting_cols.into_iter().sorted().collect();
    let interesting_rows: Vec<usize> = interesting_rows.into_iter().sorted().collect();

    let mut grid: Grid<usize> = Grid::new(interesting_rows.len(), interesting_cols.len());

    grid.transform(|(gc, val)| {
        let real_row = interesting_rows[gc.0];
        let real_col = interesting_cols[gc.1];
        *val = fixed_values
            .iter()
            .map(|&(r, c, v)| v + (abs_diff(r, real_row) + abs_diff(c, real_col)) * D)
            .min()
            .unwrap();
    });

    let mut grid_interior_sums = 0;

    //Sum top/left part only of each interior grid
    for (row, &real_row) in interesting_rows.iter().enumerate().skip(1) {
        let prev_row = row - 1;
        let prev_real_row = interesting_rows[prev_row];

        for (col, &real_col) in interesting_cols.iter().enumerate().skip(1) {
            let prev_col = col - 1;
            let prev_real_col = interesting_cols[prev_col];

            let i_n_rows = real_row - prev_real_row + 1;
            let i_n_cols = real_col - prev_real_col + 1;

            let corner_values = [
                grid[(prev_row, prev_col)],
                grid[(prev_row, col)],
                grid[(row, col)],
                grid[(row, prev_col)],
            ];
            let i_sum: usize =
                calc_grid_sum_4_influencers(&corner_values, i_n_cols, i_n_rows, D, MODULO).unwrap();
            //- corner_values.iter().sum::<usize>();
            grid_interior_sums += i_sum;

            grid_interior_sums %= MODULO;

            grid_interior_sums += corner_values[BOTTOM_RIGHT];

            //remove double counted row
            grid_interior_sums += MODULO;
            grid_interior_sums -= calc_sum_2_influencers(
                &[corner_values[BOTTOM_LEFT], corner_values[BOTTOM_RIGHT]],
                i_n_cols,
                1,
                D,
                MODULO,
            )
            .unwrap();

            //remove double counted col
            grid_interior_sums += MODULO;
            grid_interior_sums -= calc_sum_2_influencers(
                &[corner_values[TOP_RIGHT], corner_values[BOTTOM_RIGHT]],
                1,
                i_n_rows,
                D,
                MODULO,
            )
            .unwrap();
        }
    }

    //To account for single row/single col grids

    //add in the rightmost col
    for (row, &real_row) in interesting_rows.iter().enumerate().skip(1) {
        let prev_row = row - 1;
        let prev_real_row = interesting_rows[prev_row];

        let i_n_rows = real_row - prev_real_row + 1;
        let i_n_cols = 1;

        let corner_values = [grid[(prev_row, grid.C - 1)], grid[(row, grid.C - 1)]];
        let i_sum: usize =
            calc_sum_2_influencers(&corner_values, i_n_cols, i_n_rows, D, MODULO).unwrap();
        //- corner_values.iter().sum::<usize>();
        grid_interior_sums += i_sum;
        //remove double counted col
        if row < grid.R - 1 {
            grid_interior_sums += MODULO;
            grid_interior_sums -= corner_values[1] % MODULO;
        }
    }

    //add in bottom row
    for (col, &real_col) in interesting_cols.iter().enumerate().skip(1) {
        let prev_col = col - 1;
        let prev_real_col = interesting_cols[prev_col];

        let i_n_rows = 1;
        let i_n_cols = real_col - prev_real_col + 1;

        let corner_values = [grid[(grid.R - 1, prev_col)], grid[(grid.R - 1, col)]];
        let i_sum: usize =
            calc_sum_2_influencers(&corner_values, i_n_cols, i_n_rows, D, MODULO).unwrap();

        grid_interior_sums += i_sum;
        //remove double counted col
        if col < grid.C - 1 {
            grid_interior_sums += MODULO;
            grid_interior_sums -= corner_values[1] % MODULO;
        }
    }

    //remove double counted square
    if grid.R > 1 && grid.C > 1 {
        grid_interior_sums += MODULO;
        grid_interior_sums -= grid[(grid.R - 1, grid.C - 1)] % MODULO;
    }

    format!("Case #{}: {}\n", case_no, grid_interior_sums % MODULO)
}

/// Finds linear intersection between
/// `start+Di = stop+(length-i-2)*D`
/// `start+Di = stop + D(len-1) - iD`
/// `2id = stop + D(len-1) - start`
/// length-1 D increments in length, assumed that start is at 0 and stop is len-1
/// 0..1..2; len=3; 2 increments
fn find_intersection(start: usize, stop: usize, length: usize, D: usize) -> Option<usize>
{
    if !is_valid(start, stop, length, D) {
        return None;
    }
    let ret = ((length - 1) * D + stop - start) / (2 * D);
    assert!(ret <= length);
    Some(ret)
}

/// [Start, Start+D, Start+2D, Start+3D,..., Stop] in a list of length elements
fn is_valid(start: usize, stop: usize, length: usize, D: usize) -> bool
{
    if length < 1 {
        return false;
    }
    let max_diff = (length - 1) * D;
    let diff = abs_diff(start, stop);

    diff <= max_diff
}

///Sum 0 to stop
fn sum_closed_range(stop: usize) -> usize
{
    stop * (stop + 1) / 2
}

///
/// Calculates sum of a grid where a corner starts with seed value
/// and each cell is max(D+min(adj. cell value))
fn calc_rectangle_sum_single_influencer(
    seed_value: usize,
    width: usize,
    height: usize,
    D: usize,
    modulo: usize,
) -> usize
{
    //Top row is
    //seed + D+seed + 2D+seed + (width-1)*D*seed
    //which is  (sum 1 to width-1) * D + seed * width
    let top_row_sum =
        mul_mod(D, sum_closed_range(width - 1), modulo) + mul_mod(seed_value, width, modulo);
    //each row adds D*width more to each cell
    let square_sum = mul_mod(height, top_row_sum, modulo)
        + mul_mod(D * width, sum_closed_range(height - 1), modulo);

    debug!(
        "Calculated sum top left: {} width: {} height: {} D: {} == {}",
        seed_value, width, height, D, square_sum
    );
    square_sum % modulo
}

fn calc_triangle_sum(seed_value: usize, triangle_len: usize, D: usize, modulo: usize) -> usize
{
    /* we want to sum value(number of squares) + the series of D 0 + 2*D + 3*2D + 4*3D + 5*4D
    This serials is equal to D(sum (1 to num squares)^2) - D(sum 1 to num squares)
    (1² + 2² + 3² + 4² + ...) - (1+2+3+4) = 0 + 2D + 3*2D + 4*3D  which is what we want
    */

    sub_mod(
        sum_sq_to_n(triangle_len, modulo),
        sum_0_to_n(triangle_len, modulo),
        modulo,
    ) * D
        + mul_mod(sum_0_to_n(triangle_len, modulo), seed_value, modulo)
}

///assume top/left and bottom/right
fn calc_sum_2_influencers(
    seed_values: &[usize],
    width: usize,
    height: usize,
    D: usize,
    modulo: usize,
) -> Option<usize>
{
    debug!(
        "Calculating diag.  Seeds: {:?} width: {} height: {}",
        seed_values, width, height
    );

    let top_left_inf_value = seed_values[0];
    let bottom_right_inf_value = seed_values[1];

    //First check validity
    let max_diff = (height + width - 2) * D;
    let diff = (top_left_inf_value as i64 - bottom_right_inf_value as i64).abs() as usize;

    if diff > max_diff {
        return None;
    }
    if diff == max_diff {
        return Some(calc_rectangle_sum_single_influencer(
            min(top_left_inf_value, bottom_right_inf_value),
            width,
            height,
            D,
            modulo,
        ));
    }

    //Check degenerate case

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

    let (top_left_manhat_dist, rem) = div_rem(
        D * (width + height - 2) + bottom_right_inf_value - top_left_inf_value,
        2 * D,
    );
    debug!("Manhatten Distance is {} {}", top_left_manhat_dist, rem);

    //the sphere of influence starting from the bottom right corner, -1 since it doesn't overlap with TL's influence
    let br_manhat_distance = height + width - 2 - top_left_manhat_dist - 1;

    let mut top_rect_height = 0;
    let mut bot_rect_height = 0;
    let mut left_rect_width = 0;
    let mut right_rect_width = 0;
    let mut total_sum = 0;

    /*
             C0 |   C1 |   C2 |   C3 |   C4 |   C5 |   C6 |   C7 |   C8 |   C9 |
         +------+------+------+------+------+------+------+------+------+------+
      R0 | (TL) |   15 |   25 |   35 |   B  | Brev |   65 |   75 |   77 |   A  |
         +------+------+------+------+------+------+------+------+------+------+
      R1 |   15 |   25 |   35 |    B | Brev |   65 |   75 |   77 |    A | Arev |
         +------+------+------+------+------+------+------+------+------+------+
      R2 |   25 |   35 |   B  | Brev |   65 |   75 |   77 |    A | Arev |   47 |
         +------+------+------+------+------+------+------+------+------+------+
      R3 |   35 |   B  | Brev |   65 |   75 |   77 |    A | Arev |   47 |   37 |
         +------+------+------+------+------+------+------+------+------+------+
      R4 |    B | Brev |   25 |   35 |   45 |   A  | Arev |   75 |   77 |   C  |
         +------+------+------+------+------+------+------+------+------+------+
      R5 | Brev |   25 |   35 |   45 |    A | Arev |   75 |   77 |   C  |   57 |
         +------+------+------+------+------+------+------+------+------+------+
      R6 |   25 |   35 |   45 |   A  | Arev |   75 |   77 |    C |   57 |   47 |
         +------+------+------+------+------+------+------+------+------+------+
      R7 |   35 |   45 |    A | Arev |   75 |   77 |    C |   57 |   47 | (BR) |
         +------+------+------+------+------+------+------+------+------+------+

    */

    //Do we have a top rectangle (in ex. diag C would be [R0..R3]
    if top_left_manhat_dist >= width {
        top_rect_height = top_left_manhat_dist - width + 1;
        total_sum += calc_rectangle_sum_single_influencer(
            top_left_inf_value,
            width,
            top_rect_height,
            D,
            modulo,
        );
    }

    //Do we have a bottom rectangle (in ex, diag B would be [R5..R7]
    if height > 2 && top_left_manhat_dist < height - 2 {
        //In ex above, if man. hat dist is 4, then we want [R6..R7], so 8(height) -1 (start on top row) - 4(man hat dist) -1 (br. diagonal)
        bot_rect_height = height - 1 - top_left_manhat_dist - 1;
        total_sum += calc_rectangle_sum_single_influencer(
            bottom_right_inf_value,
            width,
            bot_rect_height,
            D,
            modulo,
        );
    }

    //Left retangle (in ex, diag C would be [C0..C5]; note it is sliced off by the top/bottom rectangles
    if top_left_manhat_dist >= height {
        left_rect_width = top_left_manhat_dist - height + 1;
        let seed_value = top_left_inf_value + D * top_rect_height;
        total_sum += calc_rectangle_sum_single_influencer(
            seed_value,
            left_rect_width,
            height - top_rect_height - bot_rect_height,
            D,
            modulo,
        );
    }

    //Right rectangle (in ex, diag B would be [C5..C9]
    if br_manhat_distance >= height {
        //one col is for the reverse of the triangle (see A-rev)
        //(width - 1) - (M - 1)
        right_rect_width = br_manhat_distance - height + 1;
        let seed_value = bottom_right_inf_value + D * bot_rect_height;
        //stop 1 row short of BR's diagonal
        total_sum += calc_rectangle_sum_single_influencer(
            seed_value,
            right_rect_width,
            height - top_rect_height - bot_rect_height,
            D,
            modulo,
        );
    }

    //Now the actual diagonal/triangle
    let top_left_triangle_height = min(
        top_left_manhat_dist + 1 - top_rect_height,
        height - top_rect_height,
    );
    //-1 is to take into account we have 2 columns in the jagged part
    let top_left_triangle_width = min(
        top_left_manhat_dist + 1 - left_rect_width,
        width - left_rect_width,
    );
    assert_eq!(top_left_triangle_width, top_left_triangle_height);

    let top_left_triangle_seed = top_left_inf_value + (left_rect_width + top_rect_height) * D;
    let top_left_triangle_sum =
        calc_triangle_sum(top_left_triangle_seed, top_left_triangle_height, D, modulo);
    total_sum += top_left_triangle_sum;

    let bottom_right_triangle_height = min(
        br_manhat_distance + 1 - bot_rect_height,
        height - bot_rect_height,
    );
    let bottom_right_triangle_width = min(
        br_manhat_distance + 1 - right_rect_width,
        width - right_rect_width,
    );
    assert_eq!(bottom_right_triangle_height, bottom_right_triangle_width);
    let bottom_right_triangle_seed =
        bottom_right_inf_value + (right_rect_width * D) + (bot_rect_height * D);
    let bottom_right_triangle_sum = calc_triangle_sum(
        bottom_right_triangle_seed,
        bottom_right_triangle_width,
        D,
        modulo,
    );

    total_sum += bottom_right_triangle_sum;

    //Some(total_sum )
    Some(total_sum % modulo)
}

mod constants
{
    pub const TOP_LEFT: usize = 0;
    pub const TOP_RIGHT: usize = 1;
    pub const BOTTOM_RIGHT: usize = 2;
    pub const BOTTOM_LEFT: usize = 3;

    pub const MODULO: usize = 1_000_000_007;
}

///
/// Top left, top right, bottom right, bottom left
fn calc_grid_sum_4_influencers(
    corner_values: &[usize],
    width: usize,
    height: usize,
    D: usize,
    modulo: usize,
) -> Option<usize>
{
    if height < 2 || width < 2 {
        return None;
    }
    /*Find intersection of influence of each corner
    the frontier will always be 2 rows/2cols.  Even if the intersection really is 1 row/col
    its convenient to seperate each sub square cleanly to not have to deal with duplicate sums
    */
    let top_lr_col = try_opt!(find_intersection(
        corner_values[TOP_LEFT],
        corner_values[TOP_RIGHT],
        width,
        D
    ));
    let bottom_lr_col = try_opt!(find_intersection(
        corner_values[BOTTOM_LEFT],
        corner_values[BOTTOM_RIGHT],
        width,
        D,
    ));
    let left_tb_row = try_opt!(find_intersection(
        corner_values[TOP_LEFT],
        corner_values[BOTTOM_LEFT],
        height,
        D,
    ));
    let right_tb_row = try_opt!(find_intersection(
        corner_values[TOP_RIGHT],
        corner_values[BOTTOM_RIGHT],
        height,
        D,
    ));

    let mut rows = vec![left_tb_row, right_tb_row];
    let mut cols = vec![top_lr_col, bottom_lr_col];
    rows.sort();
    cols.sort();

    // [ 0, top_lr_col ]
    let tl = calc_rectangle_sum_single_influencer(
        corner_values[TOP_LEFT],
        inclusive_range_len(0, top_lr_col),
        inclusive_range_len(0, rows[0]),
        D,
        modulo,
    );

    let tr = if top_lr_col < width - 1 {
        calc_rectangle_sum_single_influencer(
            corner_values[TOP_RIGHT],
            inclusive_range_len(top_lr_col + 1, width - 1),
            inclusive_range_len(0, rows[0]),
            D,
            modulo,
        )
    } else {
        0
    };

    let top_sum = tl + tr;

    //[rows[0]+1 .. rows[1]]
    let mid_sum = if rows[1] > rows[0] {
        //[0..cols[0]]
        //[cols[0]+1..cols[1]]

        let seed_values = if left_tb_row == rows[1] {
            [
                corner_values[TOP_LEFT] + D * inclusive_range_len(0, rows[0]),
                corner_values[BOTTOM_RIGHT] + D * inclusive_range_len(rows[1] + 1, height - 1),
            ]
        } else {
            [
                corner_values[TOP_RIGHT] + D * inclusive_range_len(0, rows[0]),
                corner_values[BOTTOM_LEFT] + D * inclusive_range_len(rows[1] + 1, height - 1),
            ]
        };
        calc_sum_2_influencers(
            &seed_values,
            inclusive_range_len(0, width - 1),
            inclusive_range_len(rows[0] + 1, rows[1]),
            D,
            modulo,
        )
        .unwrap()
    } else {
        0
    };

    let bottom_sum = if rows[1] < height - 1 {
        let bl = calc_rectangle_sum_single_influencer(
            corner_values[BOTTOM_LEFT],
            inclusive_range_len(0, bottom_lr_col),
            inclusive_range_len(rows[1] + 1, height - 1),
            D,
            modulo,
        );

        let br = if bottom_lr_col < width - 1 {
            calc_rectangle_sum_single_influencer(
                corner_values[BOTTOM_RIGHT],
                inclusive_range_len(bottom_lr_col + 1, width - 1),
                inclusive_range_len(rows[1] + 1, height - 1),
                D,
                modulo,
            )
        } else {
            0
        };
        bl + br
    } else {
        0
    };

    Some((top_sum + mid_sum + bottom_sum) % modulo)
}

//cargo test round3_d -- --nocapture
#[cfg(any(test))]
mod test_round3_d
{
    use super::*;
    use crate::util::grid::constants::*;
    use crate::util::grid::Grid;
    use crate::util::grid::*;
    use rand::distributions::{Distribution, Uniform};
    use rand::prelude::StdRng;
    use rand::SeedableRng;
    use std::ops::Range;
    use std::usize;

    ///
    /// Assume inf is in top/left corner.  each grid cell gets D added to it

    fn find_grid_sum_naive_single_influencer(
        inf_value: usize,
        width: usize,
        height: usize,
        D: usize,
    ) -> usize
    {
        let mut g: Grid<usize> = Grid::new(height, width);

        let corner_coord = IntCoord2d(0, 0);

        g[0] = inf_value;

        g.transform(|(coord, val)| {
            *val = inf_value + corner_coord.distance(&coord) * D;

            //debug!("Set val {} loc {}", val, coord);
        });

        debug!("Grid\n{:#.6?}\nD {:?}", g, D);
        // for

        g.iter_loc().map(|lv| lv.1).sum()
    }

    /// seed values in same side corners  (row0)
    pub fn find_grid_sum_naive_ranges(
        corners: &[usize],
        width: usize,
        width_range: Range<usize>,
        height: usize,
        height_range: Range<usize>,
        D: usize,
    ) -> Option<usize>
    {
        if width * height <= 1 {
            return None;
        }
        if width < 2 || height < 2 {
            return None;
        }

        let mut g: Grid<usize> = Grid::new(height, width);

        let corner_coords: Vec<_> = (0..=3)
            .map(|i| {
                IntCoord2d::<usize>(
                    if i < 2 { 0 } else { height - 1 },
                    if i == 0 || i == 3 { 0 } else { width - 1 },
                )
            })
            .collect();

        for (coord, val) in corner_coords.iter().zip(corners.iter()) {
            g[*coord] = *val;
        }

        g.transform(|(coord, val)| {
            let max_values = corner_coords
                .iter()
                .zip(corners.iter())
                .map(|(cc, val)| val + cc.distance(&coord) * D);
            *val = max_values.min().unwrap();

            //debug!("Set val {} loc {}", val, coord);
        });

        for (coord, val) in corner_coords.iter().zip(corners.iter()) {
            g[*coord] = *val;
        }

        debug!(
            "Grid\n{:#.6?}\n corners {:?}\nvalues {:?}\n \
             D {:?}\n Width range {:?} Height Range {:?}",
            g, corner_coords, corners, D, width_range, height_range
        );

        for (loc, v1) in g.iter_loc() {
            let loc: IntCoord2d<i64> = loc.convert();
            for dir in DIRECTIONS.iter() {
                if let Some(v2) = g.get_value(*dir + loc) {
                    if (*v2 as i64 - *v1 as i64).abs() > D as i64 {
                        return None;
                    }
                }
            }
        }

        Some(
            g.iter_loc()
                .filter(|(loc, _value)| {
                    loc.0 >= height_range.start
                        && loc.0 < height_range.end
                        && loc.1 >= width_range.start
                        && loc.1 < width_range.end
                })
                .map(|lv| lv.1)
                .sum(),
        )
    }

    fn find_grid_sum_naive_two_influencer(
        corners: &[usize],
        width: usize,
        height: usize,
        D: usize,
    ) -> Option<usize>
    {
        if width * height <= 1 {
            return None;
        }

        let mut g: Grid<usize> = Grid::new(height, width);

        let corner_coords = vec![IntCoord2d(0, 0), IntCoord2d(height - 1, width - 1)];

        for (coord, val) in corner_coords.iter().zip(corners.iter()) {
            g[*coord] = *val;
        }

        g.transform(|(coord, val)| {
            let max_values = corner_coords
                .iter()
                .zip(corners.iter())
                .map(|(cc, val)| val + cc.distance(&coord) * D);
            *val = max_values.min().unwrap();

            //debug!("Set val {} loc {}", val, coord);
        });

        for (coord, val) in corner_coords.iter().zip(corners.iter()) {
            g[*coord] = *val;
        }

        //debug!("Grid\n{:#.6?}\n corners {:?}\nvalues {:?}\nD {:?}", g, corner_coords, corners, D);

        for (loc, v1) in g.iter_loc() {
            let loc: IntCoord2d<i64> = loc.convert();
            for dir in DIRECTIONS.iter() {
                if let Some(v2) = g.get_value(*dir + loc) {
                    if (*v2 as i64 - *v1 as i64).abs() > D as i64 {
                        return None;
                    }
                }
            }
        }

        Some(g.iter_loc().map(|lv| lv.1).sum())
    }

    #[test]
    pub fn test_grid_sum_right_no_inf()
    {
        let D = 5;
        let corner_values = [0, 15, 25, 10];

        let sum = calc_grid_sum_4_influencers(&corner_values[..], 4, 3, D, usize::MAX);

        assert_eq!(
            sum,
            Some(5 + 10 + 15 + 5 + 10 + 15 + 20 + 10 + 15 + 20 + 25)
        );
    }

    #[test]
    pub fn test_grid_sum_height_1()
    {
        let D = 5;
        let corner_values = [0, 15];

        let sum = calc_sum_2_influencers(&corner_values[..], 4, 1, D, usize::MAX);

        assert_eq!(sum, Some(5 + 10 + 15));

        let corner_values = [0, 14];
        let sum = calc_sum_2_influencers(&corner_values[..], 7, 1, D, usize::MAX);

        assert_eq!(sum, Some(5 + 10 + 15 + 20 + 19 + 14));

        let corner_values = [2, 5];
        let sum = calc_sum_2_influencers(&corner_values[..], 2, 1, D, usize::MAX);

        assert_eq!(sum, Some(2 + 5));
    }

    #[test]
    pub fn test_grid_sum_gen()
    {
        println!("Starting...");
        let mut rng: StdRng = SeedableRng::seed_from_u64(42);

        let grid_seed_values = Uniform::from(3..50usize);
        let grid_dims = Uniform::from(1..20usize);
        for _ in 0..1000 {
            let D = grid_seed_values.sample(&mut rng);
            let corner_values: Vec<_> = (0..4).map(|_| grid_seed_values.sample(&mut rng)).collect();
            let grid_width = grid_dims.sample(&mut rng);
            let grid_height = grid_dims.sample(&mut rng);

            let sum1 = find_grid_sum_naive_ranges(
                &corner_values[..],
                grid_width,
                0..grid_width,
                grid_height,
                0..grid_height,
                D,
            );
            let sum2 = calc_grid_sum_4_influencers(
                &corner_values[..],
                grid_width,
                grid_height,
                D,
                usize::MAX,
            );

            debug!("Sum1 {:?} Sum2 {:?}", sum1, sum2);
            assert_eq!(sum1, sum2);
        }
    }
    #[test]
    pub fn test_2inf_grid_sum_gen()
    {
        println!("Starting...");
        let mut rng: StdRng = SeedableRng::seed_from_u64(42);

        let grid_seed_values = Uniform::from(3..50usize);
        let grid_dims = Uniform::from(1..20usize);
        for _ in 0..1000 {
            let D = grid_seed_values.sample(&mut rng);
            let corner_values: Vec<_> = (0..4).map(|_| grid_seed_values.sample(&mut rng)).collect();
            let grid_width = grid_dims.sample(&mut rng);
            let grid_height = grid_dims.sample(&mut rng);

            let sum1 =
                find_grid_sum_naive_single_influencer(corner_values[0], grid_width, grid_height, D);
            let sum2 = calc_rectangle_sum_single_influencer(
                corner_values[0],
                grid_width,
                grid_height,
                D,
                usize::MAX,
            );

            debug!("Sum1 {} Sum2 {}", sum1, sum2);
            assert_eq!(sum1, sum2);

            let sum3 = find_grid_sum_naive_two_influencer(
                &corner_values[0..2],
                grid_width,
                grid_height,
                D,
            );
            let sum4 = calc_sum_2_influencers(
                &corner_values[0..2],
                grid_width,
                grid_height,
                D,
                usize::MAX,
            );

            debug!("Sum3 {:?} Sum4 {:?}", sum3, sum4);
            assert_eq!(sum3, sum4);
        }
    }

}
