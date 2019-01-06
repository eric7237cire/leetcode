use super::super::util::input::*;
use std::cmp::min;

pub fn solve_all_cases()
{
    let mut reader = InputReader::new();
    let t = reader.read_int();

    for case in 1..=t {
        let (_, P) = reader.read_tuple_2::<u8, u8>();
        let mut G: Vec<_> = reader.read_int_line::<u8>();

        print!("{}", solve(case, &mut G, P));
    }
}

fn solve(case_no: u32, G: &mut Vec<u8>, P:u8) -> String
{
    debug!("Solving case {}", case_no);

    for g in G.iter_mut() {
        *g %= P;
    }

    let mut G_count = (0..P).map(|i|
        G.iter().filter(|&&g| g == i).count()
    ).collect::<Vec<_>>();

    debug!("P={} G_count={:?}", P, G_count);

    let ans = match P {
        2 => {
            let count_1 = G.iter().filter(|&&g| g == 1).count();
            let count_0 = G.len() - count_1;
            count_0 + count_1 / 2 + count_1 % 2
        },
        3 => {
            // state machine
            let NEED_1 = 2usize;
            //if group size % 3 == 1, then they need 2 leftovers
            let NEED_2: usize = 1usize;
            let mut leftover = 0u8;
            let mut groups_happy = 0;
            for i in 0..G.len() {
                //state machine
                match leftover {
                    0 => {
                        groups_happy+=1;
                        if G_count[0] > 0 {
                            G_count[0] -= 1;
                            continue;
                        }else if G_count[NEED_1] > 0 {
                            G_count[NEED_1] -= 1;
                            leftover = P-1;
                            continue;
                        }else if G_count[NEED_2] > 0 {
                            G_count[NEED_2] -= 1;
                            leftover = P-2;
                            continue;
                        } else {
                            return "Problem0".to_string();
                        }
                    },
                    1 => {
                        if G_count[NEED_1] > 0 {
                            G_count[NEED_1] -= 1;
                            leftover = 0;
                            continue;
                        }
                        else if G_count[NEED_2] > 0 {
                            G_count[NEED_2] -= 1;
                            leftover = P+1-2;
                            continue;
                        } else {
                            return "Problem1".to_string();
                        }
                    }
                    2 => {
                        if G_count[NEED_2] > 0 {
                            G_count[NEED_2] -= 1;
                            leftover = 0;
                            continue;
                        }
                        else if G_count[NEED_1] > 0 {
                            G_count[NEED_1] -= 1;
                            leftover = 2-1;
                            continue;
                        } else {
                            return "Problem2".to_string();
                        }
                    }
                    _ => {}
                }
            }

            groups_happy
        }
        _ => {
            0
        }
    };

    format!("Case #{}: {}\n", case_no, ans)
}
