pub struct Solution {

}

impl Solution {
    pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
        let mut counter = 0;
        for c in s.chars() { 
            if (j.contains(c)) {
                counter+=1;
            }
        }
        return counter;
    }
}

fn main() {
    
    let checks = [
    ( ("aA", "aAAbbbb"), 3 ),
    ( ("z", "ZZ"), 0 ) ];

    println!("Hello, world!");

    for check in checks.iter()
    {
        let solution_args = check.0;
        let expected_ans = check.1;
        let actual_ans = Solution::num_jewels_in_stones(
            solution_args.0.to_string(),
        solution_args.1.to_string());
        if (expected_ans != actual_ans) {
            println!("Problem {} != {}", actual_ans, expected_ans);
        } else {
            println!("OK {} == {}", actual_ans, expected_ans);
        
        }
    }
}