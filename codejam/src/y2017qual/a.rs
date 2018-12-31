use std::io::stdin;

pub fn solve_case() -> String
{
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let s: Vec<_> = s.split_whitespace().collect();
    let k = s[1].trim().parse::<usize>().unwrap();
    let s = s[0].trim();
    return "bob".to_string();
}