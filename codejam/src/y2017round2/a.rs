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

fn solve(case_no: u32, G: &mut Vec<u8>, P: u8) -> String
{
    debug!("Solving case {}", case_no);

    for g in G.iter_mut() {
        *g %= P;
    }

    let mut G_count = (0..P)
        .map(|i| G.iter().filter(|&&g| g == i).count())
        .collect::<Vec<_>>();

    G_count.push(0);

    debug!("P={} G_count={:?}", P, G_count);

    let ans = {
        // state machine
        let NEED_1 = P as usize - 1usize;
        //if group size % 3 == 1, then they need 2 leftovers
        let NEED_2: usize = if P >= 3 {P as usize - 2usize} else {P as usize};
        let NEED_3: usize = if P >= 4 {P as usize - 3usize} else {P as usize};
        let NEED_INDEX = [0,NEED_1,NEED_2,NEED_3];
        let mut leftover = 0u8;
        let mut groups_happy = 0;
        for _ in 0..G.len() {
            //state machine
            match leftover {
                0 => {
                    groups_happy += 1;
                    if G_count[0] > 0 {
                        G_count[0] -= 1;
                        continue;
                    }
                    else if G_count[NEED_3] > 0 && G_count[NEED_INDEX[P as usize -3]]> 0 {
                        G_count[NEED_3] -= 1;
                        leftover = P - 3;
                        continue;
                    }
                    else if G_count[NEED_2] > 0 && G_count[NEED_INDEX[P as usize -2]]> 0 {
                        G_count[NEED_2] -= 1;
                        leftover = P - 2;
                        continue;
                    }
                    else if G_count[NEED_1] > 0 {
                        G_count[NEED_1] -= 1;
                        leftover = P - 1;
                        continue;
                    } else if G_count[NEED_2] > 0 {
                        G_count[NEED_2] -= 1;
                        leftover = P - 2;
                        continue;
                    }
                    else if G_count[NEED_3] > 0 {
                        G_count[NEED_3] -= 1;
                        leftover = P - 3;
                        continue;
                    }
                    else {
                        return "Problem0\n".to_string();
                    }
                }
                1 => {
                    if G_count[NEED_1] > 0 {
                        G_count[NEED_1] -= 1;
                        leftover = 0;
                        continue;
                    }
                    //short circuit if we can have a nice leftover of 0
                    else if G_count[NEED_3] > 0 && G_count[NEED_INDEX[P as usize+1-3]] > 0 {
                        G_count[NEED_3] -= 1;
                        leftover = P + 1 - 3;
                        continue;
                    }
                    else if G_count[NEED_2] > 0 {
                        G_count[NEED_2] -= 1;
                        leftover = P + 1 - 2;
                        continue;
                    } else if G_count[NEED_3] > 0 {
                        G_count[NEED_3] -= 1;
                        leftover = P + 1 - 3;
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
                    //short circuit if we can have a nice leftover of 0
                    else if G_count[NEED_3] > 0 && G_count[NEED_INDEX[P as usize+2-3]] > 0 {
                        G_count[NEED_3] -= 1;
                        leftover = P + 2 - 3;
                        continue;
                    }
                    else if G_count[NEED_1] > 0 {
                        G_count[NEED_1] -= 1;
                        leftover = 2 - 1;
                        continue;
                    } else if G_count[NEED_3] > 0 {
                        G_count[NEED_3] -= 1;
                        leftover = P + 2 - 3;
                        continue;
                    } else {
                        return "Problem2".to_string();
                    }
                },
                3 => {
                    if G_count[NEED_3] > 0 {
                        G_count[NEED_3] -= 1;
                        leftover = 0;
                        continue;
                    }
                     else if G_count[NEED_1] > 0 && G_count[NEED_INDEX[3-1]] >0 {
                        G_count[NEED_1] -= 1;
                        leftover = 3 - 1;
                        continue;
                    }
                     else if G_count[NEED_2] > 0 {
                        G_count[NEED_2] -= 1;
                        leftover = 3 - 2;
                        continue;
                    }
                    else if G_count[NEED_1] > 0 {
                        G_count[NEED_1] -= 1;
                        leftover = 3 - 1;
                        continue;
                    } else {
                        return "Problem3".to_string();
                    }
                }
                _ => {}
            }
        }

        groups_happy
    };

    format!("Case #{}: {}\n", case_no, ans)
}
