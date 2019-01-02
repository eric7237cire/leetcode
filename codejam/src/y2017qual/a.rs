use std::io::stdin;

//use bit_vec::BitVec;

pub fn solve_case()
{
    //handle input / output
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let s: Vec<_> = s.split_whitespace().collect();
    let k = s[1].trim().parse::<usize>().unwrap();
    let s = s[0].trim();
    let mut v: Vec<bool> = s.chars().map(|x| x == '+').collect();

    match solve(&mut v, k) {
        None => println!("IMPOSSIBLE"),
        Some(ans) => println!("{}", ans),
    }
}

fn solve(pancake_row: &mut [bool], k: usize) -> Option<usize>
{
    let mut moves = 0;
    //proceed left to right, flipping as we must
    for i in 0..pancake_row.len() - k + 1 {
        if !pancake_row[i] {
            moves += 1;
            for j in i..i + k {
                pancake_row[j] = !pancake_row[j];
            }
        }
    }

    //if everything is how it should be, we succeeded
    match pancake_row.iter().all(|&x| x) {
        true => Some(moves),
        false => None,
    }
}
