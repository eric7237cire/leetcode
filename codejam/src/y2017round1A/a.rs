use std::io::stdin;
use std::thread;


pub fn solve_all_cases()
{
    let mut children: Vec<thread::JoinHandle<_>> = vec![];

    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let t = s.trim().parse::<u32>().unwrap();

    for case in 1..=t {
        //handle input / output
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();
        //debug!("Read {}", s);
        let r_and_c: Vec<u8> = s.split_whitespace().map(|n| n.parse().unwrap()).collect();
        let (r, c) = (r_and_c[0], r_and_c[1]);

        
        let mut grid: Vec<Vec<char>> = Vec::new();
        
        for _ in 0..r {
            s.clear();
            stdin().read_line(&mut s).unwrap();
            grid.push( s.chars().collect() );

        }

        if cfg!(feature = "debug_print") && case != 4 {
            continue;
        }

        children.push(thread::spawn(move || -> String {
            solve(case, &grid)
        }));
    }

    for child in children {
        print!("{}", child.join().unwrap());
    }
}

fn solve(case_no: u32, grid: &Vec<Vec<char>>) -> String {

    format!("Case #{}: bob {}\n", case_no, grid.len())
}