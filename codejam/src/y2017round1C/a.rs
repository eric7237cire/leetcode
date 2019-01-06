use super::super::util::input::*;
//use std::u16;
use std::f64;
//use std::cmp::max;

#[derive(Debug)] //,PartialEq,Eq,PartialOrd,Ord)]
struct Pancake
{
    R: u32,
    H: u32,
}

type Distance = u32;
type CityIndex = usize;

pub fn solve_all_cases()
{
    let mut reader = InputReader::new();
    let t = reader.read_int();

    for case in 1..=t
    {
        let (N, K) = reader.read_tuple_2::<u16, u16>();
        let mut pancakes: Vec<_> = (0..N)
            .map(|_| reader.read_tuple_2::<u32, u32>())
            .map(|tp| Pancake { R: tp.0, H: tp.1 })
            .collect();

        print!("{}", solve(case, &mut pancakes, K));
    }
}


fn solve(
    case_no: u32,
    pancakes: &mut Vec<Pancake>,
    K: u16
) -> String
{
    debug!("Solving case {}", case_no);

    pancakes.sort_unstable_by(|a,b| b.R.cmp(&a.R).then(b.H.cmp(&a.H)));
    debug!("Pancakes: {:?}", pancakes);

    /*for i in (1..pancakes.len()).rev() {
        if pancakes[i-1].R == pancakes[i].R {
            pancakes.remove(i);
        }
    }*/

    /*for i in K as usize..pancakes.len() {
            pancakes.pop();
    }*/

    let max_syrup:f64 =    pancakes.windows(K as usize).map( |pancakes|
        {
            let area = (pancakes[0].R as u64).pow(2) as f64 * f64::consts::PI;
            let sides: f64 = pancakes.iter().map(|p| 2f64 * f64::consts::PI * p.R as f64 * p.H as f64).sum();
            area + sides
        }
    ).fold(0f64, |acc,x| if x > acc {x} else {acc});

    debug!("Pancakes: {:?}", pancakes);

    format!("Case #{}: {}\n", case_no, max_syrup)
}
