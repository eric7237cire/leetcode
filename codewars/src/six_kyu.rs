
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