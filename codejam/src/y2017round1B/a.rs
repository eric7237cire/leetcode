use super::super::util::input::read_int_line;
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

#[derive(Debug)]
struct HorseSegment {
    start_time: f64,
    stop_time: f64,
    start_pos: f64,
    stop_pos: f64
}

impl HorseSegment {
    fn velocity(&self) -> f64 {
        (self.stop_pos - self.start_pos) / (self.stop_time - self.start_time)
    }
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
    /*if case_no != 21 {
        return "".to_string();
    }*/
    let mut horse_segs : Vec<HorseSegment> = horses.iter().map( |h| HorseSegment{start_time: 0f64, start_pos: h.0 as f64,
    stop_pos: D, stop_time: (D - h.0 as f64) / h.1 as f64}).collect();

    //Sort by starting position
    horse_segs.sort_by( |h1, h2| h1.start_pos.partial_cmp(&h2.start_pos).unwrap() );

    let mut cur_index = 0;

    while cur_index < horse_segs.len() - 1 {
        let cur = &horse_segs[cur_index];
        let next = &horse_segs[cur_index + 1];
        if next.velocity() >= cur.velocity() {
            //anything that is faster won't affect the answer
            horse_segs.remove(cur_index+1);
            continue;
        }

        //Now make sure they intersect before D
        let inter_t = (cur.start_pos-next.start_pos) / (next.velocity() - cur.velocity());        
        let inter_p = cur.start_pos + cur.velocity() * inter_t;
        //let inter_p_check = next.start_pos + next.velocity() * inter_t;
        //assert!( (inter_p - inter_p_check).abs() < 1e-6, format!("inter_p {} {}", inter_p, inter_p_check));
       /* if !( (inter_p - inter_p_check).abs() < 1e-6) {
            panic!( format!("inter_p {} {}", inter_p, inter_p_check));
        }*/

        if inter_p >= D {
            debug!("other horse finishes before: {:?} {:?}", cur, next);
            horse_segs.remove(cur_index+1);
            continue;
        }

        
        horse_segs[cur_index].stop_pos = inter_p;
        horse_segs[cur_index].stop_time = inter_t;
        
        //horse_segs[cur_index+1].start_pos = inter_p;
        //horse_segs[cur_index+1].start_time = inter_t;

        cur_index+=1;
    }

    let mut min_v = std::f64::MAX;

    for (i,hs) in horse_segs.iter().enumerate() {
        debug!("After processing, horse {} is {:?}.  V={:3}", i, hs, hs.velocity());

        let t = (D-hs.start_pos) / hs.velocity() + hs.start_time;
        let v = D / t;
        debug!("After processing, horse {} is {:?}.  V={:3}.  V to intersect={:3}", i, hs, hs.velocity(), v);
        if v < min_v {
            min_v = v;
        }
    }

    format!("Case #{}: {:.6}\n", case_no, min_v)
}
