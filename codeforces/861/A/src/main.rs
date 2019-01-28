use std::io::stdin;

fn read_token_line<T>() -> Vec<T>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        let mut s = String::new();

        stdin().read_line(&mut s).unwrap();
        s
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect()
    }

fn main()
{
    let line1 = read_token_line::<u16>();
    let t = line1[0];
    
    for _ in 0..t {
        let line = read_token_line::<i64>();
        let a = line[0];
        let b = line[1];
        let k = line[2];

        let pos = (k / 2) * (a-b) + (k%2) * a;
        println!("{}", pos);
    }
}