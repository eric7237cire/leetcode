pub struct Solution {

}

use std::collections::VecDeque;

impl Solution {
    
    pub fn shortest_subarray(a: Vec<i32>, k: i32) -> i32 {
        
        assert!(k>0);

        /*
        sum_before[i] means sum of all elements in a strictly before that index
        This is why the len is 1 more than a and the first element is 0.
        */
        let mut sum_before : Vec<i32> = Vec::new();
        sum_before.push(0);
        for x in a.iter()
        {
            let last_val : i32 = *sum_before.last().unwrap() ;
            sum_before.push(*x + last_val );
        }

        let mut best_len = a.len()+1;
        let mut start_points: VecDeque<usize> = VecDeque::new();
        
        //sum is start <= index < stop
        
        let mut start : usize = 0;
        let debug = false;
        let n:usize = a.len() ;

        //Code adapted from https://stackoverflow.com/questions/17391025/finding-a-minimal-subarray-of-n-integers-of-sum-k-in-linear-time
        for end in 0..n {
            
            let total_to_end = sum_before[end+1];

            if debug {
                println!("Starting loop.  End is {}, total is {}", end, total_to_end);
            }

            //This means can we move start to the right while still having a sum >= k?
            while !start_points.is_empty() && total_to_end - sum_before[start_points[0]] >= k 
            {
                start = start_points.pop_front().unwrap();                
                //This start means (sum of a from start <= i <= end ) >= k
                debug_assert!( sum_before[end+1] - sum_before[start] >= k );                
            }

            if debug {
                println!("Start = {} End is {}, total is {}.  start_points={:?}", start, end, total_to_end, start_points);
            }
         
            if total_to_end - sum_before[start] >= k && end-start+1 < best_len
            {
                best_len = end - start + 1
            }
            
            /*
            This is the crux of this algorithm.  Lets say we have some starting candidates:

            [2, 3, 5]<--back
            and we are currently looking at end == 5

            if sum from index 0 to 5 (==total_to_end) is less than
            the sum from index 0 to 4, then
            starting at index==5 would never be optimal (this happens when a[5] is negative)

            if sum from index 0 to 5 (==total_to_end) is less than
            the sum from index 0 to 2, then
            this means that the sum of 3 to 5 is negative, so we would never want to start from index==3

            Said another way, the property being maintained in this start_points candidate DEQ is to have
            strictly increasing cumulative sums.

            So this is very clevery removing sub arrays whose sum is 0

            */
            while !start_points.is_empty() && total_to_end <= sum_before[*start_points.back().unwrap()] // remove bad candidates
            {
                start_points.pop_back();
                if debug {
                    println!("\n!! Removed a bad candidates.  start_points={:?}", start_points);
                }
            }

            if debug {
                println!("After removing bad candidates.  Start = {} End is {}, total is {}.  start_points={:?}", start, end, total_to_end, start_points);
            }

            start_points.push_back(end+1) // end+1 is a new candidate
        }
        if best_len > a.len() {
            return -1;
        } else {
            return best_len as i32;
        }

    }
}


fn main() {
    
    let checks = [
       
      ( (vec![39353,64606,-23508,5678,-17612,40217,15351,-12613,-37037,64183,68965,-19778,-41764,-21512,17700,-23100,77370,64076,53385,30915,18025,17577,10658,77805,56466,-2947,29423,50001,31803,9888,71251,-6466,77254,-30515,2903,76974,-49661,-10089,66626,-7065,-46652,84755,-37843,-5067,67963,92475,15340,15212,54320,-5286],

207007), 4),

 ( (vec![1,2], 4), -1),

( (vec![
    12,17,26,72,93,
    95,-46,66,-38,-18,
    83,95,75,8,93,
    7,25,16,67,-19],
413), 9 ),
        ( (vec![77,19,35,10,-14], 19), 1 ),
   ( (vec![-34,37,51,3,-12,-50,51,100,-47,99,34,
   14,-13,89,31,-14,-44,23,-38,6],
   151), 2),
    ( (vec![1], 1), 1 ),
    ( (vec![1,2], 4), -1 ) ,
    ( (vec![2,-1,2], 3), 3 ) ,
    ( (vec![2,3,2,1,-1,2,4], 6), 2 ) , 
    ( (vec![1,2,3,3], 6), 2 ) , 
    ( (vec![-2,3,2,1,-1,2,4], 7), 5 )
    ];

    println!("Hello, world!");

    for check in checks.iter()
    {
        let solution_args = &check.0;
        let expected_ans = check.1;
        let actual_ans = Solution::shortest_subarray(
            solution_args.0.to_vec(),
            solution_args.1);
        if expected_ans != actual_ans {
            println!("Problem {} != {}", actual_ans, expected_ans);
        } else {
            println!("OK {} == {}", actual_ans, expected_ans);
        
        }
        //break;
    }
}