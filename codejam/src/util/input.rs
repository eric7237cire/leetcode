use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

pub struct Input<'a>
{
    source: Box<BufRead + 'a>,
}

impl<'a> Input<'a>
{
    fn console() -> Input<'a>
    {
        Input {
            source: Box::new(BufReader::new(io::stdin())),
        }
    }

    pub fn file(path: &str) -> io::Result<Input<'a>>
    {
        File::open(path).map(|file| Input {
            source: Box::new(io::BufReader::new(file)),
        })
    }
}

impl<'a> Read for Input<'a>
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>
    {
        self.source.read(buf)
    }
}

impl<'a> BufRead for Input<'a>
{
    fn fill_buf(&mut self) -> io::Result<&[u8]>
    {
        self.source.fill_buf()
    }

    fn consume(&mut self, amt: usize)
    {
        self.source.consume(amt);
    }
}

pub struct InputReader<'a>
{
    pub s: String,
    pub i: Input<'a>,
}

impl<'a> InputReader<'a>
{
    pub fn new() -> InputReader<'a>
    {
        InputReader {
            s: String::new(),
            i: Input::console(),
        }
    }

    pub fn read_num_line<T>(&mut self) -> Vec<T>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.s.clear();
        self.i.read_line(&mut self.s).unwrap();
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
        self.i.read_line(&mut self.s).unwrap();
        self.s.split_whitespace().map(|n| n.parse().unwrap())
    }

    pub fn read_int<T>(&mut self) -> T
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.s.clear();
        self.i.read_line(&mut self.s).unwrap();
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
        self.i.read_line(&mut self.s).unwrap();
        //debug!("Read line {}", self.s);
        let mut sw = self.s.split_whitespace();
        (
            sw.next().unwrap().parse::<T>().unwrap(),
            sw.next().unwrap().parse::<U>().unwrap(),
        )
    }

    pub fn read_tuple_3<T>(&mut self) -> (T, T, T)
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.s.clear();
        self.i.read_line(&mut self.s).unwrap();
        //debug!("Read line {}", self.s);
        let mut sw = self.s.split_whitespace();
        (
            sw.next().unwrap().parse::<T>().unwrap(),
            sw.next().unwrap().parse::<T>().unwrap(),
            sw.next().unwrap().parse::<T>().unwrap(),
        )
    }

    pub fn read_chars(&mut self, amt: usize) -> Vec<char>
    {
        self.s.clear();
        self.i.read_line(&mut self.s).unwrap();
        self.s.chars().take(amt).collect::<Vec<_>>()
    }

    pub fn read_string(&mut self) -> &str
    {
        self.s.clear();
        self.i.read_line(&mut self.s).unwrap();
        self.s.trim()
    }
}
