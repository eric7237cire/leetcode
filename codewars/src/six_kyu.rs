
fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
    if k == 0 || k > strarr.len() {
        return "".to_string();
    }
    let lens: Vec<usize> = strarr.iter().map( |s| s.len() ).collect();

    //initialize first sum
    let mut conseq_len : usize = lens.iter().take(k).sum();
    let mut best_len = conseq_len;
    let mut best_start_index = 0;

    //rolling sum, O(n)
    for start_index in 1..strarr.len() - k + 1 {
        conseq_len -= lens[start_index-1];
        conseq_len += lens[start_index+k-1];
        if conseq_len > best_len {
            best_len = conseq_len;
            best_start_index = start_index;
        }
    }

    (&strarr[best_start_index..best_start_index+k].join("")).to_string()
}

fn testing_longest_consec(strarr: Vec<&str>, k: usize, exp: &str) -> () {
    assert_eq!(&longest_consec(strarr, k), exp)
}

#[test]
fn basics_longest_consec() {
    testing_longest_consec(vec!["zone", "abigail", "theta", "form", "libe", "zas"], 2, "abigailtheta");
    testing_longest_consec(vec!["ejjjjmmtthh", "zxxuueeg", "aanlljrrrxx", "dqqqaaabbb", "oocccffuucccjjjkkkjyyyeehh"], 1, 
        "oocccffuucccjjjkkkjyyyeehh");
    testing_longest_consec(vec![], 3, "");
    testing_longest_consec(vec!["it","wkppv","ixoyx", "3452", "zzzzzzzzzzzz"], 3, "ixoyx3452zzzzzzzzzzzz");
    testing_longest_consec(vec!["it","wkppv","ixoyx", "3452", "zzzzzzzzzzzz"], 15, "");
    testing_longest_consec(vec!["it","wkppv","ixoyx", "3452", "zzzzzzzzzzzz"], 0, "");
}

/*
mine, 1 liner, O(N) 
fn solution(num: i32) -> i32 {
  (1..num).filter( |n| n % 3 == 0 || n % 5 == 0 ).sum()
}
*/

//Solution of https://www.codewars.com/kata/reviews/58d85666eda71d6e40000002/groups/5a0edccc27965c26f4003218 which is O(1)
fn solution(max: i32) -> i32 {
  // Result is equals to 
  // sum of numbers divided by 3 + sum of numbers divided by 5 - sum of numbers divided by 3 and 5
  
  sumOfSequence(3, max) + sumOfSequence(5, max) - sumOfSequence(15, max)
}

fn sumOfSequence(divider: i32, max: i32) -> i32 {
  // Result is sum of a sequence X, X * 2, X * 3, X * 4, ..., X * N
  // which is also equals to X * (1 + 2 + 3 + 4 + ... + N)
  // Where X is a divider and N is a highest number satisfying the
  // condition X * N < max, so N is equals to floor(max / X)
  let n = (max - 1) / divider;
  let sum_to_n = (n * (n + 1)) / 2;
  
  divider * sum_to_n
}

#[test]
fn returns_expected() {
  assert_eq!(solution(10), 23);
  assert_eq!(solution(11), 33);
  assert_eq!(solution(6), 8);
  assert_eq!(solution(std::i32::MAX/100000), 107582594);
}

fn valid_braces(s: &str) -> bool {
    let mut stack = Vec::new();
    for c in s.chars()
    {
        match c { 
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            // works because stack.pop returns an Option
             x => if Some(x) != stack.pop() { return false }
        }       
    }

    stack.is_empty()
}

fn valid_braces_expect_true(s: &str)
{
    assert!(valid_braces(s));
}
fn valid_braces_expect_false(s: &str)
{
    assert!(!valid_braces(s));
}
#[test]
fn valid_braces_tests() 
{
    valid_braces_expect_true("()");
    valid_braces_expect_false("[(])");
    valid_braces_expect_true("(){}[]");
    valid_braces_expect_true("([{}])");
    valid_braces_expect_false("(}");
    valid_braces_expect_false("[(])");
    valid_braces_expect_false("[({})](]");
    valid_braces_expect_false("(((({{");
}