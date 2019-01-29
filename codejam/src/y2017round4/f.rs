use crate::algo::vectors::*;
use crate::util::codejam::run_cases;
use crate::util::grid::Grid;
use bimap::BiMap;
use num_bigint::BigInt;
use num_traits::*;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::{SliceRandom, StdRng};
use rand::{Rng, SeedableRng};
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::i64;
use std::io::Write;
use std::mem;
use std::ops::Sub;
use std::time::Instant;

/*
*/

pub fn solve_all_cases()
{
    run_cases(
        &["F-small-practice", "F-large-practice"],
        "y2017round4",
        |reader, buffer| {
            let t = reader.read_int();

            for case in 1..=t {
                let N = reader.read_int::<usize>();
                let home = reader.read_array_3::<i64>();
                let dest = reader.read_array_3::<i64>();

                let teleporters: Vec<_> = (0..N).map(|_| reader.read_array_3::<i64>()).collect();

                if case != 22 {
                    //continue;

                }

                println!("Solving {}", case);

                //if teleporters.len() > 2 && teleporters.len() <= 6 {

                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case,
                    //if let Some(ans) = solve_small_only_U(&home, &dest, &teleporters) {
                    if let Some(ans) = solve(&home, &dest, &teleporters) {
                        format!("{}", ans)
                    } else {
                        "IMPOSSIBLE".to_string()
                    }
                )
                .unwrap();
                //}
            }
        },
    );
}

type Point = [i64; 3];

fn dist(a: &Point, b: &Point) -> i64
{
    (a[0] - b[0]).abs() + (a[1] - b[1]).abs() + (a[2] - b[2]).abs()
}

fn get_longest_path_for_step(
    dist_matrix: &Vec<Vec<Vec<i64>>>,
    home_dist: &Vec<i64>,
    steps: usize,
) -> Vec<i64>
{
    let N = dist_matrix[0].len();

    let mut ans = vec![vec![-1; N]; N];
    let mut new_ans = vec![vec![-1; N]; N];

    for step_idx in 0..dist_matrix.len() {
        if (1 << step_idx) & steps == 0 {
            continue;
        }

        if ans[0][0] == -1 {
            //println!("Initializing with {}", step_idx);
            ans = dist_matrix[step_idx].clone();
            continue;
        }

        // println!("Multiplying with {}", step_idx);
       
        //
        for t1_idx in 0..N {
            for t2_idx in 0..N {
                let mut best = -1;
                for v_idx in 0..N {
                    best = max(
                        best,
                        ans[t1_idx][v_idx] + dist_matrix[step_idx][v_idx][t2_idx],
                    );
                }

                new_ans[t1_idx][t2_idx] = best;

                /* println!("Dist matrix {} to {}, step {} = {}",
                t1_idx, t2_idx, steps_idx, best); */
            }
        }

        mem::swap(&mut ans, &mut new_ans);
    }

    /* (0..N).map(
    |t_idx| ans[t_idx].iter().max().unwrap() +
    home_dist[t_idx]).collect()*/

    //return indexed by end point
    (0..N)
        .map(|stop_idx| {
            { (0..N).map(|start_idx| ans[start_idx][stop_idx] + home_dist[start_idx]) }
                .max()
                .unwrap()
        })
        .collect()
}

fn solve(home: &Point, dest: &Point, teleporters: &Vec<Point>) -> Option<u64>
{
    let min_dist_home = teleporters
        .iter()
        .fold(i64::MAX, |acc, t| min(acc, dist(&home, t)));

    let min_dist_dest = teleporters
        .iter()
        .fold(i64::MAX, |acc, t| min(acc, dist(&dest, t)));

    let max_dist_home = teleporters
        .iter()
        .fold(i64::MIN, |acc, t| max(acc, dist(&home, t)));

    let max_dist_dest = teleporters
        .iter()
        .fold(i64::MIN, |acc, t| max(acc, dist(&dest, t)));

    let max_dist = max(max_dist_dest, max_dist_home);

    ///extra
    let mut dist_matrix = Vec::new();

    let mut max_step_idx = 1;

    /*
    create a matrix [log steps][t_idx][t2_idx]

    calculates dist[32 steps][t1][t2] =
    best of dist[16 steps][t1][v] +
    dist[16 steps][v][t2]
    */

    for steps_idx in 0..44 {
        dist_matrix.push(vec![vec![-1; teleporters.len()]; teleporters.len()]);

        if steps_idx == 0 {
            for (t1_idx, t1) in teleporters.iter().enumerate() {
                for (t2_idx, t2) in teleporters.iter().enumerate() {
                    dist_matrix[steps_idx][t1_idx][t2_idx] = dist(t1, t2);
                }
            }
        } else {
            for (t1_idx, t1) in teleporters.iter().enumerate() {
                for (t2_idx, t2) in teleporters.iter().enumerate() {
                    let mut best = -1;
                    for (v_idx, v) in teleporters.iter().enumerate() {
                        best = max(
                            best,
                            dist_matrix[steps_idx - 1][t1_idx][v_idx]
                                + dist_matrix[steps_idx - 1][v_idx][t2_idx],
                        );
                    }

                    dist_matrix[steps_idx][t1_idx][t2_idx] = best;

                    /* println!("Dist matrix {} to {}, step {} = {}",
                    t1_idx, t2_idx, steps_idx, best); */
                }
            }
        }

        let u_max = dist_matrix[steps_idx].iter().flatten().max().unwrap();

        // take care of overflow
        if *u_max >= (1i64 << 50) {
            break;
        }

        /* println!("In precalculations: After step idx {} max is {}",
        steps_idx, u_max);*/
    }

    //println!("min. d home {} dest {}", min_dist_home, min_dist_dest);

    //make home the closest one
    let (home, dest) = if min_dist_home > min_dist_dest {
        (dest, home)
    } else {
        (home, dest)
    };

    //Check if one teleport is enough, after this code only works with 2
    for t in teleporters.iter() {
        if dist(home, t) == dist(dest, t) {
            return Some(1);
        }
    }

    let mut dist_home = Vec::new();
    let mut dist_target = Vec::new();

    for t in teleporters.iter() {
        //Check if one teleport is enough
        if dist(home, t) == dist(dest, t) {
            return Some(1);
        }
        dist_home.push(dist(home, t));
        dist_target.push(dist(dest, t));
    }

    //only case where its impossible
    if teleporters.len() == 1 {
        return None;
    }

    let mut min_num_steps = 1;
    let mut max_num_steps = 1 << (dist_matrix.len() - 1);

    while max_num_steps > min_num_steps {
        let steps = (max_num_steps + min_num_steps) / 2;

        let fast_umax = get_longest_path_for_step(&dist_matrix, &dist_home, steps);

        /*
        println!("Steps {} min {} max {} umax {}\n{:?}\nDist target:\n{:?}", steps,
         min_num_steps,
         max_num_steps,
         fast_umax.iter().max().unwrap(),
         fast_umax,
         dist_target
         );*/

        let any_in_range =
            (0..teleporters.len()).any(|t_idx| dist_target[t_idx] <= fast_umax[t_idx]);

        if !any_in_range {
            min_num_steps = steps + 1;
        } else {
            max_num_steps = steps;
        }
    }

    Some(1 + max_num_steps as u64)
}

/// Tests large observation that we only need to calculate U
/// And validates iterative squaring approach to calculing max U distance per step
fn solve_small_only_U(home: &Point, dest: &Point, teleporters: &Vec<Point>) -> Option<u64>
{
    let target_distance = dist(home, dest);
    //let mut L: Vec<Vec<i64>> = Vec::new();
    //let mut U: Vec<Vec<i64>> = Vec::new();

    let target_distance = dist(home, dest);

    let min_dist_home = teleporters
        .iter()
        .fold(i64::MAX, |acc, t| min(acc, dist(&home, t)));

    let min_dist_dest = teleporters
        .iter()
        .fold(i64::MAX, |acc, t| min(acc, dist(&dest, t)));

    let max_dist_home = teleporters
        .iter()
        .fold(i64::MIN, |acc, t| max(acc, dist(&home, t)));

    let max_dist_dest = teleporters
        .iter()
        .fold(i64::MIN, |acc, t| max(acc, dist(&dest, t)));

    let max_dist = max(max_dist_dest, max_dist_home);

    ///extra
    let mut dist_matrix = Vec::new();

    for steps_idx in 0..50 {
        if (1i64 << steps_idx) > max_dist {
            break;
        }
        dist_matrix.push(vec![vec![-1; teleporters.len()]; teleporters.len()]);

        if steps_idx == 0 {
            for (t1_idx, t1) in teleporters.iter().enumerate() {
                for (t2_idx, t2) in teleporters.iter().enumerate() {
                    dist_matrix[steps_idx][t1_idx][t2_idx] = dist(t1, t2);
                }
            }
        } else {
            for (t1_idx, t1) in teleporters.iter().enumerate() {
                for (t2_idx, t2) in teleporters.iter().enumerate() {
                    let mut best = -1;
                    for (v_idx, v) in teleporters.iter().enumerate() {
                        best = max(
                            best,
                            dist_matrix[steps_idx - 1][t1_idx][v_idx]
                                + dist_matrix[steps_idx - 1][v_idx][t2_idx],
                        );
                    }

                    dist_matrix[steps_idx][t1_idx][t2_idx] = best;

                    /* println!("Dist matrix {} to {}, step {} = {}",
                    t1_idx, t2_idx, steps_idx, best); */
                }
            }
        }

        println!(
            "After step idx {} max is {}",
            steps_idx,
            dist_matrix[steps_idx].iter().flatten().max().unwrap()
        );
    }

    println!("min. d home {} dest {}", min_dist_home, min_dist_dest);

    let (home, dest) = if min_dist_home > min_dist_dest {
        (dest, home)
    } else {
        (home, dest)
    };

    for t in teleporters.iter() {
        //Check if one teleport is enough
        if dist(home, t) == dist(dest, t) {
            return Some(1);
        }
    }

    let mut initial = Vec::new();
    for (t_idx, t) in teleporters.iter().enumerate() {
        /*println!("Teleporter #{}, dist home: {}",
        t_idx, dist(home, t));*/
        initial.push(dist(home, t));
    }

    let mut L = initial.clone();
    let mut U = initial.clone();

    /*
    By definition, Lt,i+1 and Ut,i+1 are the distances from t to its closest and farthest points in Ri, respectively.
     The farthest point in Ri from t is at a distance which is the maximum over all teleporters u of dist(t, u) + Uu,i
     (this is the distance to the point on the surface of the sphere centered at u with radius Uu,
     i that is the opposite direction from t).
    */
    for i in 1..10000 {
        if i < 68 {
            println!("i {} max is {}", i, U.iter().max().unwrap());
        }

        let fast_umax = get_longest_path_for_step(&dist_matrix, &initial, i - 1);

        if i > 1 {
            let fast_umax_all = fast_umax.iter().max().unwrap();
            let current_umax = U.iter().max().unwrap();

            assert_eq!(*current_umax + 1, *fast_umax_all);

            /*println!("maxes: {} and {}\nU vs fast: {:?} ",
            fast_umax_all,current_umax, fast_umax.iter().zip(U.iter()).collect::<Vec<_>>());
            */
        }

        let mut new_L = Vec::new();
        let mut new_U = Vec::new();

        for (t_idx, t) in teleporters.iter().enumerate() {
            if i > 1 {
                let current_umax = U[t_idx];
                let fast_umax_t = fast_umax[t_idx];
                assert_eq!(current_umax, fast_umax_t);
            }

            if
            //dist(&dest, t) >= L[t_idx] &&
            i > 1 && dist(&dest, t) <= U[t_idx] {
                return Some(i as u64);
            }

            if teleporters.len() == 1 {
                return None;
            }

            /*           println!("Starting iteration #{}, teleporter #{} U[{}] = {}",
            i, t_idx, t_idx, U[t_idx]);*/

            let mut low = None;
            let mut high = None;
            for (u_idx, u) in teleporters.iter().enumerate() {
                if u_idx == t_idx {
                    continue;
                }
                //Greatest distance from teleporter u + distance of t to u;
                //this is the furthest one could teleport using teleporter t
                let maybe_high = U[u_idx] + dist(u, t);
                if high.is_none() || maybe_high > high.unwrap() {
                    high = Some(maybe_high);
                }

                /*
                                . For each teleporter u we need to consider:

                dist(t, u) - Uu,i if dist(t, u) > Uu,i (t is outside the outer sphere centered at u),
                Lu,i - dist(t, u) if dist(t, u) < Lu,i (t is inside the inner sphere), or
                0, in all other cases (t is in between, that is, it is itself a reachable point).
                */

                let dist_tu = dist(t, u);
                let maybe_low = if dist_tu > U[u_idx] {
                    //lowest distance is outside the outer sphere
                    dist_tu - U[u_idx]
                } else if dist_tu < L[u_idx] {
                    //teleport to lower sphere
                    L[u_idx] - dist_tu
                } else {
                    0
                };

                if low.is_none() || maybe_low < low.unwrap() {
                    low = Some(maybe_low);
                }
            }

            new_L.push(low.unwrap());
            new_U.push(high.unwrap());
        }

        mem::swap(&mut L, &mut new_L);
        mem::swap(&mut U, &mut new_U);
    }

    None
}
