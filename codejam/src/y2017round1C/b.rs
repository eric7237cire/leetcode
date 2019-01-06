use super::super::util::input::*;
//use std::u16;
use std::f64;
use std::cmp::min;

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

const day : usize = 24 * 60;

#[derive(Clone,PartialEq)]
enum Parent {
    Cameron,
    Jamie
}

use self::Parent::*;

fn solve(
    case_no: u32,
    C: &[(u16,u16)],
    J: &[(u16,u16)]
) -> String
{
    debug!("Solving case {}", case_no);

    //From alkjash python solution
    let mut fixed:Vec<Option<Parent>> = vec![None; day];
    for c in C {
        //intervals are open on right
        for t in c.0..c.1 {
            fixed[t as usize] = Some(Cameron)
        }
    }
    for j in J {
        for t in j.0..j.1 {
            fixed[t as usize] = Some(Jamie);
        }
    }


    /*
     Compute F[T][C][X] by dynamic programming, which is the minimal number of swaps needed for:
    # Deciding who does each of first T times
    # C <= T of that time goes to Cameron
    # X = 0 if Cameron does the last slot
    # X = 1 if Jamie does the last slot
    */
    let mut F = vec![[ [(1+day) as u16;2] ; 1+day/2]; 1+day];
    //Stack overflow if we attempt to declare this on the stack
    //let mut F = [[ [(1+day) as u16;2] ; 1+day/2]; 1+day];

    F[0][0][0] = 0;
    F[0][0][1] = 0;
    for t in 1..=day {
        for c in 0..=day / 2 {
            if fixed[t - 1] != Some(Cameron) && c>0 { // # Cameron is free for this minute
                //so cameron does this, either we need to switch from jamie or continue
                F[t][c][0] = min(F[t - 1][c - 1][1] + 1, F[t - 1][c - 1][0]);
            }
            if fixed[t - 1] != Some(Jamie) { // # Jamie is free for this minute
                F[t][c][1] = min(F[t - 1][c][1], F[t - 1][c][0] + 1);
            }
        }
    }

    let mut ans = min(F[day][day/2][0], F[day][day/2][1]);
    if ans % 2 == 1 {
        ans += 1;
    }

    format!("Case #{}: {}\n", case_no, ans)
}
