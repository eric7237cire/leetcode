use crate::algo::vectors::*;
use crate::util::codejam::run_cases;
use crate::util::grid::Grid;
use num_bigint::BigInt;
use num_traits::*;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::{SliceRandom, StdRng};
use rand::{Rng, SeedableRng};
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::ops::Sub;
use std::time::Instant;

/*
Cross product / dot product
normal vectors in a plane
Dividing plane
Sphere
integer math
big ints

Idea: Looking at eatmore's solution, using dot*cross product https://mathinsight.org/scalar_triple_product
and maybe using interior of 3d polygons
*/

pub fn solve_all_cases()
{
    /*
    N, the number of cities visited by K. The next N lines contain three integers Xi, Yi and Zi e
    */

    run_cases(
        &["D-small-practice", "D-large-practice"],
        "y2017round4",
        |reader, buffer| {
            let t = reader.read_int();

            for case in 1..=t {
                let N = reader.read_int();

                let points = (0..N)
                    .map(|_| reader.read_tuple_3())
                    .map(|tup| [tup.0, tup.1, tup.2])
                    .collect();
                if case != 82 {
                    // continue;
                }

                writeln!(buffer, "{}", solve(case, &points)).unwrap();
            }
        },
    );
}

fn solve(case_no: u32, points: &Vec<Vector3<i64>>) -> String
{
    println!("Case {}.  # of points: {}", case_no, points.len());

    let mut points = points.clone();
    points.sort();
    points.dedup();

    let mut rng: StdRng = SeedableRng::seed_from_u64(42);

    //speed up checks
    rng.shuffle(&mut points);

    for i in 0..points.len() {
        //println!("Point {}={:#?}", i, points[i]);
        for j in 0..i {
            //a plane defined by origin + point i & j.
            //The cross product of AB + AC (where A is origin)
            //is normal to the plane
            let normal = vec3_cross(&points[i], &points[j]);

            if normal == [0, 0, 0] {
                continue;
            }

            let mut coplanar = Vec::new();

            let mut pos_count = 0;
            let mut neg_count = 0;
            let mut zero_count = 0;
            for p in points.iter() {
                //the dot product determines if the point is on one side of the plane
                let dot = vec3_dot(&normal, p);
                if dot > 0 {
                    pos_count += 1;
                } else if dot < 0 {
                    neg_count += 1;
                } else {
                    zero_count += 1;
                    coplanar.push(*p);
                }

                //Short circuit
                if pos_count > 0 && neg_count > 0 {
                    break;
                }
            }

            if pos_count == 0 || neg_count == 0 {
                //Special handling for the coplanar case
                if zero_count > 2 {
                    //One of the points make a line from origin which we rotate to hit the 2nd point
                    if check_coplanar(&coplanar, &points[i], &points[j], &normal)
                        && check_coplanar(&coplanar, &points[j], &points[i], &normal)
                    {
                        //the coplanar points cannot be seperated with a dividing line
                        continue;
                    }
                }

                return format!("Case #{}: NO", case_no);
            }
        }
    }
    format!("Case #{}: YES", case_no)
}

fn to_debug_string(a: &Vector3<BigInt>) -> String
{
    format!(
        "({}, {}, {})",
        a[0].to_str_radix(10),
        a[1].to_str_radix(10),
        a[2].to_str_radix(10)
    )
}

fn check_coplanar(
    points: &[Vector3<i64>],
    point: &Vector3<i64>,
    line: &Vector3<i64>,
    normal_to_plane: &Vector3<i64>,
) -> bool
{
    let point: Vector3<BigInt> = vec3_cast_bigint(&point);
    let line: Vector3<BigInt> = vec3_cast_bigint(&line);
    let normal_to_plane: Vector3<BigInt> = vec3_cast_bigint(&normal_to_plane);

    debug!(
        "Point: {:#?} Line: {:#?}",
        to_debug_string(&point),
        to_debug_string(&line)
    );

    let zero = BigInt::zero();

    //This is both normal to the plane containing the points and the line
    let normal_to_line = vec3_cross_ref(&normal_to_plane, &line);

    debug!(
        "Perp: {:#?} Normal: {:#?}",
        to_debug_string(&normal_to_plane),
        to_debug_string(&normal_to_line)
    );

    //they are perpendicular.  normal should be on the plane
    debug_assert!(vec3_dot_ref(&line, &normal_to_line) == zero);
    debug_assert!(vec3_dot_ref(&line, &normal_to_plane) == zero);

    let mut pos_count = 0;
    let mut neg_count = 0;
    let mut zero_count = 0;
    for p in points.iter() {
        let p: Vector3<BigInt> = vec3_cast_bigint(&p);

        debug_assert!(vec3_dot_ref(&p, &normal_to_plane) == zero);

        let dot = vec3_dot_ref(&normal_to_line, &p);

        debug!("Looking at point: {} dot: {}", to_debug_string(&p), dot);

        if dot > zero {
            pos_count += 1;
        } else if dot < zero {
            neg_count += 1;
        } else {
            zero_count += 1;
        }
    }

    assert!(points.len() > 2);

    //deal with colinear case.  If there are colinear points, then we the dividing line if
    //we rotate it will have one of the colinear points to the other side.

    if zero_count <= 1 && (pos_count == 0 || neg_count == 0) {
        //All the points are on one side of the dividing line
        return false;
    }

    return true;
}

#[cfg(test)]
mod test_2017_round4_d
{
    use super::*;

    #[test]
    fn test_plane_direction()
    {
        //flat case
        let p1 = [3, 7, 0];
        let p2 = [-2, -4, 0];

        let normal = vec3_cross(&p1, &p2);

        let mut rng: StdRng = SeedableRng::seed_from_u64(42);
        let xy_gen = Uniform::from(-100..100i64);
        let z_neg_gen = Uniform::from(-100..0i64);

        let z_pos_gen = Uniform::from(1..101i64);

        for _ in 0..1000 {
            let x = xy_gen.sample(&mut rng);
            let y = xy_gen.sample(&mut rng);
            let z = z_neg_gen.sample(&mut rng);

            let vector = [x, y, z];
            let dot = vec3_dot(&normal, &vector);
            assert!(
                dot < 0,
                format!("Dot product of {:#?} and {:#?} is {}", normal, vector, dot)
            );
        }

        for _ in 0..1000 {
            let x = xy_gen.sample(&mut rng);
            let y = xy_gen.sample(&mut rng);
            let z = z_pos_gen.sample(&mut rng);

            let vector = [x, y, z];
            let dot = vec3_dot(&normal, &vector);
            assert!(
                dot > 0,
                format!("Dot product of {:#?} and {:#?} is {}", normal, vector, dot)
            );
        }
    }
}
