//code adapted from https://www.geeksforgeeks.org/generate-unique-partitions-of-an-integer/
use std::usize;
use std::collections::BTreeSet;
fn part(n: i64) -> String {
    assert!(n<=50);
    let mut prod : BTreeSet<i64> = BTreeSet::new();
    let mut p = vec![0; n as usize]; // An array to store a partition 
    let mut k : usize = 0;  // Index of last element in a partition 
    p[k] = n;  // Initialize first partition as number itself 
  
    // This loop first prints current partition, then generates next 
    // partition. The loop stops when the current partition has all 1s 
    loop
    {         
        prod.insert( p.iter().take(k+1).product());
  
        // Generate next partition 
  
        // Find the rightmost non-one value in p[]. Also, update the 
        // rem_val is set to # of 1s 
        let mut rem_val = 0; 
        while k != usize::MAX && p[k] == 1
        { 
            rem_val += p[k]; 
            k = match k.checked_sub(1) {
                Some(ok) => ok,
                None => usize::MAX
            }
        } 
  
        // if k < 0, all the values are 1 so there are no more partitions 
        if k == usize::MAX {
            break;
        } 
  
        // Decrease the p[k] found above and adjust the rem_val 
        p[k]-=1; 
        rem_val+=1; 
    
        // If rem_val is more, then the sorted order is violated.  Divide 
        // rem_val in different values of size p[k] and copy these values at 
        // different positions after p[k] 
        while rem_val > p[k]
        { 
            p[k+1] = p[k]; 
            rem_val = rem_val - p[k]; 
            k+=1; 
        } 
  
        // Copy rem_val to next position and increment position 
        p[k+1] = rem_val; 
        k+=1; 
    }

    let avg:f64 = ((prod.iter().sum::<i64>()) as f64) / (prod.len() as f64);
    let min = prod.iter().next().unwrap();
    let max = *prod.iter().next_back().unwrap();
    let mut it = prod.iter().skip( (prod.len()-1) / 2 );
    let mut median:f64 = *it.next().unwrap() as f64;
    if prod.len() % 2 == 0 {
        median += *it.next().unwrap() as f64;
        median /= 2f64;
    }
    
    format!("Range: {} Average: {:.2} Median: {:.2}", max-min, avg, median)
}



fn testequal(ans: &str, sol: &str) {
  assert!(ans == sol, "Expected \"{}\", got \"{}\".", sol, ans);
}

#[test]
fn returns_expected() {
      testequal(&part(1), "Range: 0 Average: 1.00 Median: 1.00");
      testequal(&part(2), "Range: 1 Average: 1.50 Median: 1.50");
      testequal(&part(3), "Range: 2 Average: 2.00 Median: 2.00");
      testequal(&part(4), "Range: 3 Average: 2.50 Median: 2.50");
      testequal(&part(5), "Range: 5 Average: 3.50 Median: 3.50");
}        