use super::super::util::input::*;

use crate::algo::graph::disjointset::DisjointSet;
use std::io::Write;
use std::time::Instant;
use std::cmp::min;
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

        let tours = (0..2 * C)
            .map(|_| reader.read_tuple_3())
            .collect::<Vec<_>>();

        print!("{}", solve(case, C, &tours));
    }

    let duration = now.elapsed();
    let secs = f64::from(duration.as_secs() as u32) + f64::from(duration.subsec_nanos()) / 1e9f64;
    let _ = writeln!(
        ::std::io::stderr(),
        "\nElapsed time {:.2} second(s)\n",
        secs
    );
}
fn solve(case_no: u32, C: usize, tour_input: &Vec<(usize, usize, usize)>) -> String
{
    debug!("\n\n\nSolving case {}", case_no);



    format!("Case #{}: {}\n", case_no)
}

///
/// Top left, top right, bottom right, bottom left
fn find_grid_sum( corners: &[usize], width: usize, height: usize, D: usize, modulo: usize) -> usize
{

}

#[cfg(test)]
mod test_round3
{
    use super::*;

    fn find_grid_sum_naive( corners: &[usize], width: usize, height: usize, D: usize, modulo: usize) -> usize
    {
        let g = Grid::new();
    }

    #[test]
    fn test_merging()
    {
        let mut ds = DisjointSet::new(4);
        ds.merge_sets(3, 1);

        assert_eq!(3, ds.number_of_sets());

        ds.merge_sets(0,3);

        assert_eq!(2, ds.number_of_sets());

        ds.merge_sets(1,2);

        assert_eq!(1, ds.number_of_sets());
    }

    /*---- Test suite ----*/
    #[test]
    fn test_is_free()
    {
        let tours = vec![
            Tour {
                start_camp: 1,
                stop_camp: 0,
                leave_time: 9,
                duration: 2,
            },
            Tour {
                start_camp: 1,
                stop_camp: 0,
                leave_time: 20,
                duration: 3,
            },
            Tour {
                start_camp: 0,
                stop_camp: 1,
                leave_time: 17,
                duration: 3,
            },
            Tour {
                start_camp: 0,
                stop_camp: 1,
                leave_time: 8,
                duration: 3,
            },
        ];

        let camp = Camp {
            arrivals: vec![0, 1],
            departures: vec![2, 3],
        };

        assert_eq!(6, camp.wait_time(0, 0, &tours));

        //arrive 23:00, leave 8:00
        assert_eq!(9, camp.wait_time(1, 1, &tours));

        assert_eq!(18, camp.wait_time(1, 0, &tours));
        assert_eq!(21, camp.wait_time(0, 1, &tours));
    }
}
