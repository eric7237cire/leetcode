use crate::algo::graph::disjointset::DisjointSet;
use crate::util::codejam::run_cases;
use std::cmp::min;
use std::io::Write;

/*
Union find / disjoint sets
Clock / modular arithmetic
*/
pub fn solve_all_cases()
{
    run_cases(
        &["C-small-practice", "C-large-practice"],
        "y2017round3",
        |reader, buffer| {
            let t = reader.read_int();

            for case in 1..=t {
                let C = reader.read_int();

                let tours = (0..2 * C)
                    .map(|_| reader.read_tuple_3())
                    .collect::<Vec<_>>();

                write!(buffer, "{}", solve(case, C, &tours)).unwrap();
            }
        },
    );
}
#[derive(Debug)]
struct Tour
{
    start_camp: usize,
    stop_camp: usize,
    leave_time: usize,
    duration: usize,
}
#[derive(Debug)]
struct Camp
{
    arrivals: Vec<usize>,
    departures: Vec<usize>,
}

impl Camp
{
    fn wait_time(&self, arrival_idx: usize, departure_idx: usize, tours: &[Tour]) -> usize
    {
        (48 + tours[self.departures[departure_idx]].leave_time
            - tours[self.arrivals[arrival_idx]].leave_time
            - tours[self.arrivals[arrival_idx]].duration % 24)
            % 24
    }
    fn is_free(&self, tours: &[Tour]) -> bool
    {
        let p1 = self.wait_time(0, 0, &tours) + self.wait_time(1, 1, &tours);
        let p2 = self.wait_time(0, 1, &tours) + self.wait_time(1, 0, &tours);
        p1 == p2
    }
}
fn solve(case_no: u32, C: usize, tour_input: &[(usize, usize, usize)]) -> String
{
    debug!("\n\n\nSolving case {}", case_no);

    let tours: Vec<_> = tour_input
        .iter()
        .enumerate()
        .map(|(i, &(E, L, D))| Tour {
            start_camp: (i + 2) / 2,
            stop_camp: E,
            leave_time: L,
            duration: D,
        })
        .collect();

    debug!("Tours: {:?} ", tours);

    let camps: Vec<_> = (1..=C)
        .map(|c| Camp {
            arrivals: tours
                .iter()
                .enumerate()
                .filter(|(_, tour)| tour.stop_camp == c)
                .map(|(idx, _)| idx)
                .collect(),
            departures: tours
                .iter()
                .enumerate()
                .filter(|(_, tour)| tour.start_camp == c)
                .map(|(idx, _)| idx)
                .collect(),
        })
        .collect();

    let waiting_times: Vec<_> = camps
        .iter()
        .map(|camp| {
            //calculate wait time
            let p1 = camp.wait_time(0, 0, &tours) + camp.wait_time(1, 1, &tours);
            let p2 = camp.wait_time(0, 1, &tours) + camp.wait_time(1, 0, &tours);
            (p1, p2)
        })
        .collect();

    //debug!("Camps: {:?}", camps);

    let mut min_time = std::usize::MAX;

    for start_config in 0..4 {
        let start_arrival = start_config & 1;
        let start_depart = (start_config >> 1) & 1;

        let mut ds = DisjointSet::new(tours.len());

        //we will need to pass through the base camp with the non final arrival/departure
        ds.merge_sets(
            camps[0].arrivals[start_arrival ^ 1],
            camps[0].departures[start_depart ^ 1],
        );

        for (camp_index, camp) in camps.iter().enumerate().skip(1) {
            if waiting_times[camp_index].0 == waiting_times[camp_index].1 {
                //doesn't matter which pairing
                ds.merge_sets(camp.arrivals[0], camp.arrivals[1]);
                ds.merge_sets(camp.departures[0], camp.departures[1]);
                ds.merge_sets(camp.departures[0], camp.arrivals[1]);
            } else if waiting_times[camp_index].0 < waiting_times[camp_index].1 {
                ds.merge_sets(camp.arrivals[0], camp.departures[0]);
                ds.merge_sets(camp.arrivals[1], camp.departures[1]);
            } else {
                ds.merge_sets(camp.arrivals[0], camp.departures[1]);
                ds.merge_sets(camp.arrivals[1], camp.departures[0]);
            }
        }

        debug!(
            "Number of cycles: {}  {} {} tours: {}",
            ds.number_of_sets(),
            start_arrival,
            start_depart,
            tours.len()
        );

        let mut time = 0;
        time += waiting_times
            .iter()
            .skip(1)
            .map(|wt| min(wt.0, wt.1))
            .sum::<usize>();
        time += tours.iter().map(|t| t.duration).sum::<usize>();
        time += 24 * (ds.number_of_sets() - 1);
        time += tours[camps[0].departures[start_depart]].leave_time;
        time += camps[0].wait_time(start_arrival ^ 1, start_depart ^ 1, &tours);

        min_time = min(time, min_time);
    }

    format!("Case #{}: {}\n", case_no, min_time)
}

#[cfg(test)]
mod test_round3
{
    use super::*;

    #[test]
    fn test_merging()
    {
        let mut ds = DisjointSet::new(4);
        ds.merge_sets(3, 1);

        assert_eq!(3, ds.number_of_sets());

        ds.merge_sets(0, 3);

        assert_eq!(2, ds.number_of_sets());

        ds.merge_sets(1, 2);

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
