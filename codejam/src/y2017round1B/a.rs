use super::super::util::input::read_int_line;
use std::cmp;
use std::io::stdin;
use std::thread;

struct Horse(u64, u64);

pub fn solve_all_cases()
{
    let mut children: Vec<thread::JoinHandle<_>> = vec![];

    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let t = s.trim().parse::<u32>().unwrap();

    for case in 1..=t
    {
        //D & N
        let input: Vec<u64> = read_int_line();        
        let n = input[1];
        let horse = (0..n).map( |_| {
            let input: Vec<u64> = read_int_line();        
            Horse(input[0], input[1])
        }).collect::<Vec<_>>();

        children.push(thread::spawn(move || -> String {
            solve(
                case, input[0] as f64, &horse
            )
        }));
    }

    for child in children
    {
        print!("{}", child.join().unwrap());
    }
}

struct HorseSegment {
    start_time: f64,
    stop_time: f64,
    start_pos: f64,
    stop_pos: f64
}

impl HorseSegment {

}

fn seg_intersect<T>( seg1: &(T, T), seg2: &(T, T) ) -> bool
where T : std::cmp::PartialOrd
{
    assert!(seg1.0 <= seg1.1);
    assert!(seg2.0 <= seg2.1);
    //https://stackoverflow.com/questions/3269434/whats-the-most-efficient-way-to-test-two-integer-ranges-for-overlap
    return seg1.0 <= seg2.1 && seg2.0 <= seg1.1  ;
} 

#[test]
fn test_seg_intersect() {
    assert!(!seg_intersect::<u64>( &(1,2), &(3,4) ));
    assert!(!seg_intersect::<i64>( &(-4,-3), &(-2,-1) ));

    assert!(seg_intersect::<u8>( &(1,14), &(3,5) ), "seg2 fully inside seg1");
    assert!(seg_intersect::<f64>( &(3.1,3.2), &(2.9,5.1) ), "seg1 fully inside seg2");

    assert!(seg_intersect::<u8>( &(1,4), &(4,5) ), "endpoint shared");
    assert!(seg_intersect::<i8>( &(-1,3), &(-4,-1) ), "endpoint shared");
}

#[allow(non_snake_case)]
fn solve(case_no: u32, D: f64, horses: &Vec<Horse>) -> String
{
    let mut horse_segs = horses.iter().map( |h| HorseSegment{start_time: 0f64, start_pos: h.0 as f64,
    stop_pos: D, stop_time: (D - h.0 as f64) / h.1 as f64});

    for _ in 0..10 {
        let mut found_intersection = false;
        let mut new_horse_segs : Vec<HorseSegment> = Vec::new();
        for i in 0..horse_segs.len() {
            for j in 1..horse_segs.len() {

            }
        }
    }

    let mut ans = format!("Case #{}:\n", case_no);
    
    ans
}
