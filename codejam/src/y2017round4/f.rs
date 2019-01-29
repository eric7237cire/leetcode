use crate::algo::vectors::*;
use crate::util::codejam::run_cases;
use crate::util::grid::Grid;
use num_bigint::BigInt;
use num_traits::*;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::{SliceRandom, StdRng};
use rand::{Rng, SeedableRng};
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::Write;
use std::ops::Sub;
use std::time::Instant;
use bimap::BiMap;
use std::mem;
use std::i64;
use std::cmp::{min,max};


/*
*/

pub fn solve_all_cases()
{

    run_cases(
        &[
            "F-small-practice",
            //"F-large-practice"
        ],
        "y2017round4",
        |reader, buffer| {
            
            let t = reader.read_int();

            for case in 1..=t {
                let N  = reader.read_int::<usize>();
                let home = reader.read_array_3::<i64>();
                let dest = reader.read_array_3::<i64>();

                let teleporters:Vec<_> = (0..N).map(|_| reader.read_array_3::<i64>()).collect();
                
                if case != 12 {
                    //continue;

                }

                println!("Solving {}", case);

                //if teleporters.len() > 2 && teleporters.len() <= 6 {

                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case,
                    if let Some(ans) = solve_small_only_U(&home, &dest, &teleporters) {
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

fn dist( a: &Point, b: &Point) -> i64 
{
    (a[0]-b[0]).abs() + (a[1]-b[1]).abs() + (a[2]-b[2]).abs()
}

fn get_longest_path_for_step( dist_matrix: &Vec<Vec<Vec<i64>>>, home_dist: &Vec<i64>, steps: usize) -> i64
{
    let N = dist_matrix[0].len();

    let mut ans = vec![ vec![ -1; N ]; N ];

    for step_idx in 0..dist_matrix.len()
    {
        if (1 << step_idx) & steps == 0 {
            continue;
        }

        if ans[0][0] == -1 {
            //println!("Initializing with {}", step_idx);
            ans = dist_matrix[step_idx].clone();
            continue;
        }

        // println!("Multiplying with {}", step_idx);
        let mut new_ans = vec![ vec![ -1; N ]; N ];
        //
        for t1_idx in 0..N {
                for t2_idx in 0..N {
                    let mut best = -1;
                    for v_idx in 0..N {
                    
                        best = max( best,
                        ans[t1_idx][v_idx] +
                         dist_matrix[step_idx][v_idx][t2_idx]);
                    }

                    new_ans[t1_idx][t2_idx] = best;

                   /* println!("Dist matrix {} to {}, step {} = {}",
                    t1_idx, t2_idx, steps_idx, best); */
                }
            }

        mem::swap(&mut ans, &mut new_ans);

    }

    (0..N).map(
                |t_idx| ans[t_idx].iter().max().unwrap() + 
                home_dist[t_idx]).max().unwrap() 

}

fn solve(home: &Point, dest: &Point, teleporters: &Vec<Point>) -> Option<u64>
{
    /*
    
8 steps A->Z

4 steps A->B 
4 steps B->Z

2 steps B->C
2 steps C->Z


16 steps T1->T2
8 steps T1->V
8 steps V->T2
*/

    //create a matrix [log steps][t_idx][t2_idx]
    let mut dist_matrix = Vec::new();

    for steps_idx in 0..10 {
        dist_matrix.push(vec![ vec![ -1; teleporters.len() ]; teleporters.len() ]);

        if steps_idx == 0 {
            for (t1_idx, t1) in teleporters.iter().enumerate() {
                for (t2_idx, t2) in teleporters.iter().enumerate() {
                    dist_matrix[steps_idx][t1_idx][t2_idx] = dist( t1,t2);
                }
            }
        } else {
            for (t1_idx, t1) in teleporters.iter().enumerate() {
                for (t2_idx, t2) in teleporters.iter().enumerate() {
                    let mut best = -1;
                    for (v_idx, v) in teleporters.iter().enumerate() {
                    
                        best = max( best, dist_matrix[steps_idx-1][t1_idx][v_idx] +
                        dist_matrix[steps_idx-1][v_idx][t2_idx]);
                    }

                    dist_matrix[steps_idx][t1_idx][t2_idx] = best;
                }
            }
        }
    }



   // let target_distance = dist(home, dest);

    let min_dist_home = teleporters.iter().fold( i64::MAX,
    |acc, t| min(acc, dist(&home, t)));

    let min_dist_dest = teleporters.iter().fold( i64::MAX,
    |acc, t| min(acc, dist(&dest, t)));

    println!("min. d home {} dest {}", min_dist_home, min_dist_dest);

    //make sure home is the closest point
    let (home,dest) = if min_dist_home > min_dist_dest {
        (dest,home)
    } else { (home, dest) };
    

    let mut dist_home = Vec::new();
         
    for t in teleporters.iter() {
        //Check if one teleport is enough
        if dist(home, t) == dist(dest, t) {
            return Some(1);
        }
        dist_home.push( dist(home, t) );
    }
    /*
    Let us now consider the case where there exists
     two teleporters t and u such that dist(P, t) ≥ dist(Q, t) and dist(P, u) ≤ dist(Q, u). 
     Consider the sphere A centered at t that passes through P, and the sphere B centered at u
      that passes through Q. By the assumed inequalities, 
      A contains Q and B contains P, which means A and B intersect.
       Let x be any point at the intersection, for which dist(P, t) = dist(x, t) 
       and dist(Q, u) = dist(x, u) hold. Then, x is a possible intermediate stop to go
        from P to Q in exactly 2 teleportations, so, if the inequalities hold, 2 is the answer. 
        Notice there are other cases in which 2 is also the answer, which are covered below.

    t is closer to Q
    u is closer to P 

    t...Q...P  == sphere A
    u...P...Q  == sphere B

    A contains Q 
    B contains P 

    where sphere A & B intersect:
    dist(P,t) == dist(x,t)
    dist(Q,u) == dist(x,u)

At this point, we can assume that either P is closer to any teleporter than Q, or vice versa
 (otherwise, we can choose two teleporters to fullfill the inequalities at the beginning of 
 the previous paragraph). 
Since the problem is symmetric, swap P and Q if needed to make P the closest of P and Q to all teleporters.

    Now recall the definitions of R, L and U from the Small solution. 
    Since P is closest to all teleporters, dist(Q, t) > Ut,1 = dist(P, t) for all t.
     This means Q is outside the spheres centered in all teleporters. 
     Since Lt,i is non-increasing with i, the inner sphere contracts with each step, 
     which means Q is never inside the inner sphere, so as soon as Q is inside the
      outer sphere, we are guaranteed that Q is reachable. So, we only need to 
      calculate the Us. 
      
      By reading its definition above, we note that Ut,i is 
      equal to the longest path from P to t using teleporters as intermediate steps,
       where the length of each step is simply the distance between the two points.
    */

    let min_num_steps = 2;
    let max_num_steps = 10000;

    while max_num_steps > min_num_steps
    {
        let steps = (max_num_steps + min_num_steps) / 2;

        //init step 1
        let mut U = dist_home.clone();
        let mut new_U = Vec::new();

        for (t_idx, t) in teleporters.iter().enumerate() {

            let mut high = None ;
            for (t2_idx, t2) in teleporters.iter().enumerate() 
            {
                if t_idx == t2_idx {
                    continue;
                }

                let maybe_high = U[t2_idx] + dist(t, t2);
                if high.is_none() || maybe_high > high.unwrap() {
                    high = Some(maybe_high);
                }
            }

            new_U.push(high.unwrap());


            /*
time by using something similar to iterated squaring 
to calculate the matrix of largest distances 
from any teleporter to any other in i - 1 steps, 
and then combining that with the vector of distances 
from P to each teleporter.
 The "multiplication" here is not an actual matrix times
  matrix multiplication, but rather the use of the property 
  that the longest path from t to u in i steps is the longest 
  path from t to v in j steps plus the longest path 
  from v to u in k - j steps, for some v. 
  Taking j = k / 2 for even k shows how to do 
  log k steps overall. 
             */

            /*
t -> u in i steps
t -> v in j steps + v->u in k-j steps for some v 

i = j + k - j
            */
            
        }

    }

    None 
}

fn solve_small_only_U(home: &Point, dest: &Point, teleporters: &Vec<Point>) -> Option<u64>
{
    
    let target_distance = dist(home, dest);
    //let mut L: Vec<Vec<i64>> = Vec::new();
    //let mut U: Vec<Vec<i64>> = Vec::new();


    let target_distance = dist(home, dest);

    let min_dist_home = teleporters.iter().fold( i64::MAX,
    |acc, t| min(acc, dist(&home, t)));

    let min_dist_dest = teleporters.iter().fold( i64::MAX,
    |acc, t| min(acc, dist(&dest, t)));

    let max_dist_home = teleporters.iter().fold( i64::MIN,
    |acc, t| max(acc, dist(&home, t)));

    let max_dist_dest = teleporters.iter().fold( i64::MIN,
    |acc, t| max(acc, dist(&dest, t)));

    let max_dist = max(max_dist_dest, max_dist_home);

    ///extra
    let mut dist_matrix = Vec::new();

    for steps_idx in 0..50 {
        if (1i64 << steps_idx) > max_dist {
            break;
        }
        dist_matrix.push(vec![ vec![ -1; teleporters.len() ]; teleporters.len() ]);

        if steps_idx == 0 {
            for (t1_idx, t1) in teleporters.iter().enumerate() {
                for (t2_idx, t2) in teleporters.iter().enumerate() {
                    dist_matrix[steps_idx][t1_idx][t2_idx] = dist( t1,t2);
                }
            }
        } else {
            for (t1_idx, t1) in teleporters.iter().enumerate() {
                for (t2_idx, t2) in teleporters.iter().enumerate() {
                    let mut best = -1;
                    for (v_idx, v) in teleporters.iter().enumerate() {
                    
                        best = max( best, dist_matrix[steps_idx-1][t1_idx][v_idx] +
                        dist_matrix[steps_idx-1][v_idx][t2_idx]);
                    }

                    dist_matrix[steps_idx][t1_idx][t2_idx] = best;

                   /* println!("Dist matrix {} to {}, step {} = {}",
                    t1_idx, t2_idx, steps_idx, best); */
                }
            }
        }

        println!("After step idx {} max is {}",
        steps_idx, dist_matrix[steps_idx].iter().flatten().max().unwrap());
    }

///extra

    println!("min. d home {} dest {}", min_dist_home, min_dist_dest);

    let (home,dest) = if min_dist_home > min_dist_dest {
        (dest,home)
    } else { (home, dest) };
    
        
    for t in teleporters.iter() {
        //Check if one teleport is enough
        if dist(home, t) == dist(dest, t) {
            return Some(1);
        }
     
    }

    
    let mut initial = Vec::new();
    for (t_idx,t) in teleporters.iter().enumerate() {
        /*println!("Teleporter #{}, dist home: {}",
        t_idx, dist(home, t));*/
        initial.push( dist(home, t) );
    }
    
    let mut L = initial.clone();
    let mut U = initial.clone();

    /*
    By definition, Lt,i+1 and Ut,i+1 are the distances from t to its closest and farthest points in Ri, respectively.
     The farthest point in Ri from t is at a distance which is the maximum over all teleporters u of dist(t, u) + Uu,i 
     (this is the distance to the point on the surface of the sphere centered at u with radius Uu,
     i that is the opposite direction from t).
    */
    for i in 1..10000
    {
        
        if i < 68 {
        println!("i {} max is {}",
        i, U.iter().max().unwrap());
        }

        if i > 1 {
            let pre_max1 = (0..teleporters.len()).map(
                |t_idx| dist_matrix[0][t_idx].iter().max().unwrap() + 
                dist(&home, &teleporters[t_idx])).max().unwrap();
            let pre_max2 = (0..teleporters.len()).map(
                |t_idx| dist_matrix[1][t_idx].iter().max().unwrap() + 
                dist(&home, &teleporters[t_idx])).max().unwrap();

        let current_umax = U.iter().max().unwrap();
        let fast_umax = get_longest_path_for_step(&dist_matrix, &initial, i-1);
        assert_eq!(*current_umax, fast_umax);
        }

       

        let mut new_L = Vec::new();
        let mut new_U = Vec::new();

        for (t_idx, t) in teleporters.iter().enumerate() {

            if //dist(&dest, t) >= L[t_idx] &&
            i>1 &&
               dist(&dest, t) <= U[t_idx] {
                return Some(i as u64);
            }
       
            if teleporters.len()==1 {
                return None;
            }

/*           println!("Starting iteration #{}, teleporter #{} U[{}] = {}",
            i, t_idx, t_idx, U[t_idx]);*/

                

            let mut low = None;
            let mut high = None;    
            for (u_idx, u) in teleporters.iter().enumerate()
            {
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

                let dist_tu = dist(t,u);
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


fn solve_small(home: &Point, dest: &Point, teleporters: &Vec<Point>) -> Option<u64>
{
    let target_distance = dist(home, dest);
    //let mut L: Vec<Vec<i64>> = Vec::new();
    //let mut U: Vec<Vec<i64>> = Vec::new();

    let mut initial = Vec::new();
    for t in teleporters.iter() {
        initial.push( dist(home, t) );
    }
    let mut L = initial.clone();
    let mut U = initial.clone();

    /*
    By definition, Lt,i+1 and Ut,i+1 are the distances from t to its closest and farthest points in Ri, respectively.
     The farthest point in Ri from t is at a distance which is the maximum over all teleporters u of dist(t, u) + Uu,i 
     (this is the distance to the point on the surface of the sphere centered at u with radius Uu,
     i that is the opposite direction from t).
    */
    for i in 1..10000
    {
        let mut new_L = Vec::new();
        let mut new_U = Vec::new();

        for (t_idx, t) in teleporters.iter().enumerate() {

            if dist(&dest, t) >= L[t_idx] &&
               dist(&dest, t) <= U[t_idx] {
                return Some(i);
            }
       
            if teleporters.len()==1 {
                return None;
            }

            let mut low = None;
            let mut high = None;    
            for (u_idx, u) in teleporters.iter().enumerate()
            {
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

                let dist_tu = dist(t,u);
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
