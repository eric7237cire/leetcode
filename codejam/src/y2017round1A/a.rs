use std::io::stdin;
use std::thread;

pub fn solve_all_cases()
{
    let mut children: Vec<thread::JoinHandle<_>> = vec![];

    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let t = s.trim().parse::<u32>().unwrap();

    for case in 1..=t
    {
        //handle input / output
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();
        //debug!("Read {}", s);
        let r_and_c: Vec<u8> = s.split_whitespace().map(|n| n.parse().unwrap()).collect();
        let (r, _) = (r_and_c[0], r_and_c[1]);

        let mut grid: Vec<Vec<char>> = Vec::new();

        for _ in 0..r
        {
            s.clear();
            stdin().read_line(&mut s).unwrap();
            grid.push(s.chars().collect());
        }

        if cfg!(feature = "debug_print") && case != 4
        {
            continue;
        }

        children.push(thread::spawn(move || -> String { solve(case, &mut grid) }));
    }

    for child in children
    {
        print!("{}", child.join().unwrap());
    }
}

fn handle_col(r: usize, c: usize, grid: &mut Vec<Vec<char>>, last_value: &mut char)
{
    let value = grid[r][c];

    if value != '?'
    {
        *last_value = value;
        return;
    }

    if *last_value != '?' && value == '?'
    {
        grid[r][c] = *last_value;
    }
}

fn solve(case_no: u32, grid: &mut Vec<Vec<char>>) -> String
{
    //let mut last_value: char = '?';

    let n_rows = grid.len();
    let n_cols = grid[0].len();

    for c in 0..n_cols
    {
        let mut last_value = '?';
        for r in 0..n_rows
        {
            handle_col(r, c, grid, &mut last_value);
        }

        last_value = '?';
        for r in (0..n_rows).rev()
        {
            handle_col(r, c, grid, &mut last_value);
        }
    }

    // Now handle blank columns

    // Copy to right
    for c in 1..n_cols
    {
        if grid[0][c] == '?'
        {
            for r in 0..n_rows
            {
                grid[r][c] = grid[r][c - 1];
            }
        }
    }

    // Copy to left (from right)
    for c in (0..n_cols - 1).rev()
    {
        if grid[0][c] == '?'
        {
            for r in 0..n_rows
            {
                grid[r][c] = grid[r][c + 1];
            }
        }
    }

    let mut ans = format!("Case #{}:\n", case_no);
    for r in 0..n_rows
    {
        for c in 0..n_cols
        {
            ans += &grid[r][c].to_string();
        }
    }
    ans
}
