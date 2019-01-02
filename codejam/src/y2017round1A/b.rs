use std::io::stdin;
use std::thread;
use super::super::util::input::{read_int_line};

pub fn solve_all_cases()
{
    let mut children: Vec<thread::JoinHandle<_>> = vec![];

    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let t = s.trim().parse::<u32>().unwrap();

    for case in 1..=t
    {
        //handle input / output
        let n_and_p: Vec<u8> = read_int_line();
        let (n, p) = (n_and_p[0], n_and_p[1]);
        
        let r: Vec<u32> = read_int_line();

        let mut q: Vec<Vec<u32>> = Vec::new();
        for _ in 0..n {
            q.push(read_int_line());
        }
        children.push(thread::spawn(move || -> String { solve(case, n,p,&r, &q) }));
    }

    for child in children
    {
        print!("{}", child.join().unwrap());
    }
}

fn solve(case_no: u32, n: u8, p:u8, r: &Vec<u32>, q:&Vec<Vec<u32>>) -> String
{
    
    let mut ans = format!("Case #{}:\n", case_no);
    ans
}
