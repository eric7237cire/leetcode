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

                let teleporters = (0..N).map(|_| reader.read_array_3::<i64>()).collect();
                
                if case != 12 {
                    //continue;

                }

                println!("Solving {}", case);

                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case,
                    if let Some(ans) = solve(&home, &dest, &teleporters) {
                        format!("{}", ans)
                    } else {
                        "IMPOSSIBLE".to_string()
                    }
                )
                .unwrap();
            }
        },
    );
}

type Point = [i64; 3];

fn dist( a: &Point, b: &Point) -> i64 
{
    (a[0]-b[0]).abs() + (a[1]-b[1]).abs() + (a[2]-b[2]).abs()
}

fn solve(home: &Point, dest: &Point, teleporters: &Vec<Point>) -> Option<u64>
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

            if //dist(&dest, t) >= L[t_idx] &&
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
