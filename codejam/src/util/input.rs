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

pub struct InputReader
{
    s: String,
}

impl InputReader
{
    pub fn new() -> InputReader
    {
        InputReader { s: String::new() }
    }

    pub fn read_num_line<T>(&mut self) -> Vec<T>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.s.clear();
        stdin().read_line(&mut self.s).unwrap();
        self.s
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect()
    }

    pub fn read_int_line_iter<T: std::marker::Sized + std::str::FromStr>(
        &mut self,
    ) -> impl Iterator<Item = T> + '_
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.s.clear();
        stdin().read_line(&mut self.s).unwrap();
        self.s.split_whitespace().map(|n| n.parse().unwrap())
    }

    pub fn read_int<T>(&mut self) -> T
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.s.clear();
        stdin().read_line(&mut self.s).unwrap();
        self.s.trim().parse::<T>().unwrap()
    }

    pub fn read_tuple_2<T, U>(&mut self) -> (T, U)
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
        U: std::str::FromStr,
        <U as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.s.clear();
        stdin().read_line(&mut self.s).unwrap();
        //debug!("Read line {}", self.s);
        let mut sw = self.s.split_whitespace();
        (
            sw.next().unwrap().parse::<T>().unwrap(),
            sw.next().unwrap().parse::<U>().unwrap(),
        )
    }

    pub fn read_tuple_3<T, U, V>(&mut self) -> (T, U, V)
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
        U: std::str::FromStr,
        <U as std::str::FromStr>::Err: std::fmt::Debug,
        V: std::str::FromStr,
        <V as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.s.clear();
        stdin().read_line(&mut self.s).unwrap();
        //debug!("Read line {}", self.s);
        let mut sw = self.s.split_whitespace();
        (
            sw.next().unwrap().parse::<T>().unwrap(),
            sw.next().unwrap().parse::<U>().unwrap(),
            sw.next().unwrap().parse::<V>().unwrap(),
        )
    }

    pub fn read_chars(&mut self, amt: usize) -> Vec<char>
    {
        self.s.clear();
        stdin().read_line(&mut self.s).unwrap();
        self.s.chars().take(amt).collect::<Vec<_>>()
    }
}
