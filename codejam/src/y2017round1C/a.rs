use super::super::util::input::*;
//use std::u16;
use std::f64;
//use std::cmp::max;

/*
geometry,
brute force search
*/

#[derive(Debug)] //,PartialEq,Eq,PartialOrd,Ord)]
struct Pancake
{
    R: u32,
    H: u32,
    area_top: f64,
    area_sides: f64,
}

pub fn solve_all_cases()
{
    let mut reader = InputReader::new();
    let t = reader.read_int();

    for case in 1..=t {
        let (N, K) = reader.read_tuple_2::<u16, u16>();
        let mut pancakes: Vec<_> = (0..N)
            .map(|_| reader.read_tuple_2::<u32, u32>())
            .map(|tp| Pancake {
                R: tp.0,
                H: tp.1,
                area_top: 0f64,
                area_sides: 0f64,
            })
            .collect();

        print!("{}", solve(case, &mut pancakes, K));
    }
}

fn solve(case_no: u32, pancakes: &mut Vec<Pancake>, K: u16) -> String
{
    debug!("Solving case {}", case_no);

    pancakes.sort_unstable_by(|a, b| b.R.cmp(&a.R).then(b.H.cmp(&a.H)));

    //Precompute cylindar side surface area & top area
    for p in pancakes.iter_mut() {
        p.area_top = (p.R as u64).pow(2) as f64 * f64::consts::PI;
        p.area_sides = 2f64 * f64::consts::PI * p.R as f64 * p.H as f64;
    }

    //try all bottom pancakes, this uniquely determines the top area
    let max_syrup: f64 = (0..pancakes.len() as u16 - K + 1)
        .map(|bottom_pancake_index| {
            //get max K-1 sides
            let mut side_areas: Vec<f64> = pancakes
                .iter()
                .skip(bottom_pancake_index as usize + 1)
                .map(|p| p.area_sides)
                .collect();

            side_areas.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());

            //return area which is the heighest K-1 pancakes
            side_areas.iter().take(K as usize - 1).sum::<f64>()
                + pancakes[bottom_pancake_index as usize].area_sides
                + pancakes[bottom_pancake_index as usize].area_top
        })
        .fold(0f64, |acc, x| if x > acc { x } else { acc });

    format!("Case #{}: {}\n", case_no, max_syrup)
}
