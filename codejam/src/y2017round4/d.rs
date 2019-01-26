use num_bigint::BigInt;
use crate::util::codejam::run_cases;
use crate::util::grid::Grid;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::{StdRng, SliceRandom};
use rand::{SeedableRng,Rng};
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::time::Instant;
use std::ops::{Sub};
use crate::algo::vectors::*;
use num_traits::*;

/*

*/

pub fn solve_all_cases()
{
    /*
     N, the number of cities visited by K. The next N lines contain three integers Xi, Yi and Zi e
     */

    run_cases(
        &["D-small-practice","D-large-practice"],
        "y2017round4",
        |reader, buffer| {
            let t = reader.read_int();

            for case in 1..=t {
                let N = reader.read_int();

                let points = (0..N).map(|_| reader.read_tuple_3()).
                map( |tup| [tup.0, tup.1, tup.2]).
                collect();
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
            
    rng.shuffle(&mut points);

    for i in 0..points.len()
    {
        //println!("Point {}={:#?}", i, points[i]);
        for j in 0..i
        {
            //line is from origin to i, then hit point j
            let normal = vec3_cross(&points[i], &points[j]);

            if normal == [0,0,0] {
                continue;
            }

            let mut coplanar = Vec::new();

            let mut pos_count = 0;
            let mut neg_count = 0;
            let mut zero_count = 0;
            for p in points.iter() { 
                let dot = vec3_dot(&normal, p);
                if dot > 0 {
                    pos_count += 1;
                } else if dot < 0 {
                    neg_count += 1;
                } else {
                    zero_count += 1;
                    coplanar.push(*p);
                }
                if pos_count > 0 && neg_count > 0 {
                    break;
                }
                
            }

            if pos_count == 0 || neg_count == 0 {
                if zero_count > 2 {
                    if check_coplanar(&coplanar, &points[i], &points[j]) 
                    && check_coplanar(&coplanar, &points[j], &points[i]) 
                    {
                        continue;
                    }
                    
                    return format!("Case #{}: NO",case_no);
                } else {
                    return format!("Case #{}: NO",case_no);
                }
            }
        }
    }
    format!("Case #{}: YES",case_no)
}

fn unit_normal_triangle( a: &Vector3<i64>, b: &Vector3<i64>, c: &Vector3<i64> ) -> Vector3<f64> {
    let cp = vec3_cross( &vec3_sub(b, a), &vec3_sub(c,b) );

    let cp_f64 : Vector3<f64> = vec3_cast(&cp);

    return vec3_normalized(&cp_f64);
}

fn to_debug_string(a: &Vector3<BigInt>) -> String 
{
    format!("({}, {}, {})", a[0].to_str_radix(10),
    a[1].to_str_radix(10),
    a[2].to_str_radix(10) )
}


const EPSILON:f64 = 0.0001;
fn check_coplanar(points: &[Vector3<i64>], point: &Vector3<i64>, line: &Vector3<i64> ) -> bool {

    let point: Vector3<BigInt> = vec3_cast_bigint(&point);
    let line: Vector3<BigInt> = vec3_cast_bigint(&line);

    debug!("Point: {:#?} Line: {:#?}", to_debug_string(&point), 
    to_debug_string(&line));

    let zero = BigInt::zero();

    let perp = vec3_cross_ref(&point, &line);
    let normal = vec3_cross_ref(&perp, &line);

    debug!("Perp: {:#?} Normal: {:#?}", to_debug_string(&perp), 
    to_debug_string(&normal));
    

    //they are perpendicular.  normal should be on the plane
    assert_eq!(vec3_dot_ref(&line, &normal), zero);
    assert_eq!(vec3_dot_ref(&line, &perp), zero);

    let mut pos_count = 0;
    let mut neg_count = 0;
    let mut zero_count = 0;
    for p in points.iter() { 
        let p: Vector3<BigInt> = vec3_cast_bigint(&p);

        assert_eq!(vec3_dot_ref(&p, &perp), zero);

        let dot = vec3_dot_ref(&normal, &p);

        debug!("Looking at point: {} dot: {}",
        to_debug_string(&p), dot);

        if dot > zero {
            pos_count += 1;
        } else if dot < zero {
            neg_count += 1;
        } else {
            zero_count += 1;
        }

        
    }

    //assert_eq!(zero_count, 1);
    assert!(points.len() > 2);

    //deal with colinear case
    if zero_count <= 1 && (pos_count == 0 || neg_count == 0) {
        //all to 1 side
        return false;
    }

    /*
    for i in 1..points.len() {
        for j in 1..i {
            let n1 = unit_normal_triangle( &points[0], &points[i], &[0,0,0]);
            let n2 = unit_normal_triangle( &points[i], &points[j], &[0,0,0]);
            let n3 = unit_normal_triangle( &points[j], &points[0], &[0,0,0]);

            if (1f64-vec3_dot(&n1, &n2).abs()) < EPSILON && (1f64-vec3_dot(&n2, &n3).abs()) < EPSILON {
                return false;
            }
        }
    }*/

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
        
        for _ in 0..1000
        {
        
            let x = xy_gen.sample(&mut rng);
            let y = xy_gen.sample(&mut rng);
            let z = z_neg_gen.sample(&mut rng);

            let vector = [x,y,z];
            let dot = vec3_dot(&normal, &vector);
            assert!(dot < 0, format!("Dot product of {:#?} and {:#?} is {}", normal, vector, dot));
        }

        for _ in 0..1000
        {
        
            let x = xy_gen.sample(&mut rng);
            let y = xy_gen.sample(&mut rng);
            let z = z_pos_gen.sample(&mut rng);

            let vector = [x,y,z];
            let dot = vec3_dot(&normal, &vector);
            assert!(dot > 0, format!("Dot product of {:#?} and {:#?} is {}", normal, vector, dot));
        }
    }
}