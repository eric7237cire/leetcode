use super::super::util::input::*;
//use std::u16;
use std::f64;
//use std::cmp::max;

pub fn solve_all_cases()
{
    let mut reader = InputReader::new();
    let t = reader.read_int();

    for case in 1..=t
    {
        let (Ac, Aj) = reader.read_tuple_2::<u8, u8>();
        let C: Vec<_> = (0..Ac)
            .map(|_| reader.read_tuple_2::<u16, u16>())
            .collect();
        let J: Vec<_> = (0..Aj)
            .map(|_| reader.read_tuple_2::<u16, u16>())
            .collect();
        print!("{}", solve(case, &C, &J));
    }
}


fn solve(
    case_no: u32,
    C: &[(u16,u16)],
    J: &[(u16,u16)]
) -> String
{
    debug!("Solving case {}", case_no);

    format!("Case #{}: \n", case_no)
}
