use std::io::stdin;

pub fn read_int_line<T>() -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    s.split_whitespace().map(|n| n.parse().unwrap()).collect()
}

pub struct InputReader {
    s: String
}

impl InputReader {
    pub fn new() -> InputReader {
        InputReader { s: String::new() }
    }

    pub fn read_int_line_iter<T: std::marker::Sized + std::str::FromStr>(&mut self) -> impl Iterator<Item=T> + '_
        where
            T: std::str::FromStr,
            <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        stdin().read_line(&mut self.s).unwrap();
        self.s.split_whitespace().map(|n| n.parse().unwrap())
    }

    pub fn read_int<T>(&mut self) -> T
     where T: std::str::FromStr,
            <T as std::str::FromStr>::Err: std::fmt::Debug,

    {
        stdin().read_line(&mut self.s).unwrap();
        self.s.trim().parse::<T>().unwrap()
    }
}
