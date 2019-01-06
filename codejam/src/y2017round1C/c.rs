use super::super::util::input::*;
use std::cmp::min;


#[derive(Clone,PartialEq)]
enum Parent {
    Cameron,
    Jamie
}
use self::Parent::*;

pub fn solve_all_cases()
{
    let mut reader = InputReader::new();
    let t = reader.read_int();

    for case in 1..=t
    {
        let (Ac, Aj) = reader.read_tuple_2::<u8, u8>();

        let mut fixed:Vec<Option<Parent>> = vec![None; DAY];
        for i in 0..Ac+Aj {
            let (start,stop) = reader.read_tuple_2::<u16, u16>();
            //intervals are open on right
            for t in start..stop {
                fixed[t as usize] = if i < Ac {Some(Cameron)} else {Some(Jamie)};
            }
        }
        print!("{}", solve(case, &fixed));
    }
}

fn solve(
    case_no: u32,
) -> String
{
    debug!("Solving case {}", case_no);

    format!("Case #{}: \n", case_no)
}
