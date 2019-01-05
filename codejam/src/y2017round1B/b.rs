use super::super::util::input::read_int_line;
use super::super::util::log::init_log;
use std::fmt;
use std::io::stdin;
use std::iter::FromIterator;
//use std::slice::Iter;
use itertools::Itertools;

//use std::thread;

pub fn solve_all_cases()
{
    //let mut children: Vec<thread::JoinHandle<_>> = vec![];

    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let t = s.trim().parse::<u32>().unwrap();

    for case in 1..=t
    {
        //N, R, O(RY), Y, G(YB), B, and V(RB).
        let input: Vec<u16> = read_int_line();

        //  children.push(thread::spawn(move || -> String { solve(case, &input) }));
        print!("{}", solve(case, &input));
    }
    /*
    for child in children
    {
        print!("{}", child.join().unwrap());
    }*/
}

#[derive(Debug, Copy, Clone)]
enum Colors
{
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Violet,
}

impl Colors
{
    fn to_index(self) -> usize
    {
        match self
        {
            Red => 0,
            Orange => 1,
            Yellow => 2,
            Green => 3,
            Blue => 4,
            Violet => 5,
        }
    }
    fn to_color_binary(self) -> u8
    {
        match self
        {
            Red => 0b0_001_u8,
            Orange => 0b0_011_u8,
            Yellow => 0b0_010_u8,
            Green => 0b0_110_u8,
            Blue => 0b0_100_u8,
            Violet => 0b0_101_u8,
        }
    }
    fn to_char(self) -> char
    {
        match self
        {
            Red => 'R',
            Orange => 'O',
            Yellow => 'Y',
            Green => 'G',
            Blue => 'B',
            Violet => 'V',
        }
    }
    fn is_ok(self, other: Colors) -> bool
    {
        self.to_color_binary() & other.to_color_binary() == 0
    }
}

use self::Colors::*;
static COLORS: [Colors; 6] = [Red, Orange, Yellow, Green, Blue, Violet];

impl ::std::fmt::Display for Colors
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", self.to_char())
    }
}
/*
impl Colors
{
    pub fn iterator() -> Iter<'static, Colors>
    {
        COLORS.into_iter()
    }
}*/

struct Counts {
    total: u16,
    count: [u16;6],
} 

impl Counts 
{
    fn new() -> Counts {
        Counts{ total: 0, count: [0; 6]}
    }
    fn get_count(&self, c: Colors) -> u16 {
        self.count[c.to_index()]
    } 
    
    fn adj_count(&mut self, c: Colors, v: i16) {
        
        self.count[c.to_index()] =  (self.count[c.to_index()] as i16 + v) as u16;
        self.total = (self.total as i16 + v) as u16;
    }
    fn max_color(&self) -> Colors 
    {
        let max_color_index = self.count
                .iter()
                .enumerate()
                .max_by_key(|&(_, item)| item)
                .unwrap()
                .0;
        COLORS[max_color_index]
    }

    fn max_color_ok(&self, c1: Colors, c2: Option<Colors>) -> Option<Colors>
    {
        let max_color_index = self.count
                .iter()
                .enumerate()
            .filter( |&(_, count)| *count > 0)
            .filter( |&(idx, _)| COLORS[idx].is_ok(c1) && (c2.is_none() || COLORS[idx].is_ok(c2.unwrap())))
                .max_by_key(|&(_, count)| count);

        match max_color_index {
            None => None,
            Some(iv) => Some(COLORS[iv.0])
        }
    }
}

impl<'a> FromIterator<&'a u16> for Counts
{
    fn from_iter<I: IntoIterator<Item = &'a u16>>(iter: I) -> Self
    {
        let mut c: Counts = Counts::new();
        let mut i = 0;
        let mut n = 0;
        for v in iter
        {
            c.count[i] = *v;
            i += 1;
            n += *v;
        }
        c.total = n;
        c
    }
}

#[test]
fn test_helper1()
{
    //init_log();
    let mut sol: Vec<Colors> = vec![Red, Yellow, Blue, Red, Yellow];
    let mut counts: Counts = Counts::new();
    counts.adj_count(Blue, 1);
    let r = primary_color_sol(&mut sol, &mut counts);
    assert!(r, "snht");
}

#[test]
fn test_helper2()
{
    //init_log();
    let mut sol: Vec<Colors> = vec![Red, Red];
    let mut counts = Counts::new();
    counts.adj_count(Blue,  1);
    counts.adj_count(Yellow, 1);
    let r = primary_color_sol(&mut sol, &mut counts);
    assert!(r, "sam");
}
#[test]
fn test_helper3()
{
    //init_log();
    let mut sol: Vec<Colors> = vec![];
    let mut counts = Counts::new();
    counts.adj_count(Blue,  2);
    counts.adj_count(Yellow, 4);
    counts.adj_count(Red, 2);
    let r = primary_color_sol(&mut sol, &mut counts);
    assert!(r, "bob");
    assert_eq!(8, sol.len());
}


fn primary_color_sol(sol: &mut Vec<Colors>, counts: &mut Counts) -> bool
{
    let N = counts.total;
    sol.clear();

    let color1 = counts.max_color();

    if N == 1 {
        sol.push(color1);
        return true;
    }

    if counts.get_count(color1) > N / 2 //floor N/2
    {
        return false;
    }

    let color2 = counts.max_color_ok(color1, None);
    let color3 = counts.max_color_ok(color1, color2);

    if N <= 2 {
        sol.clear();
        sol.push(color1);
        if let Some(c2) = color2  {
            sol.push(c2);
        }
        return true;
    }

    let color2 = color2.unwrap();
    let mut pass1:Vec<Colors> = Vec::new();

    let pass1_size = N / 2 + N % 2;
    for _ in 0..counts.get_count(color1) {
        pass1.push(color1);
        counts.adj_count(color1, -1);
    }
    for _ in 0..pass1_size as usize-pass1.len() {
        pass1.push(color2);
        counts.adj_count(color2, -1);
    }

    let mut pass2:Vec<Colors> = Vec::new();
    //let pass2_size = n - pass1_size;

    for _ in 0..counts.get_count(color2) {
        pass2.push(color2);
        counts.adj_count(color2, -1);
    }

    if let Some(c3) = color3 {
        for _ in 0..counts.get_count(c3) {
            pass2.push(c3);
            counts.adj_count(c3, -1);
        }
    }
    assert_eq!(pass1.len(), pass1_size as usize);
    assert_eq!(pass2.len(), N as usize - pass1_size as usize);
    sol.extend( pass1.iter().interleave(pass2.iter()) );

    assert_eq!(sol.len(), N as usize);
    assert!(sol.first().unwrap().is_ok(*sol.last().unwrap()));

    for w in sol.windows(2) {
        assert!( w[0].is_ok(w[1]), format!("{} can't be next to {} {:?}", w[0], w[1], sol) );
    }



    true
}


#[allow(non_snake_case)]
fn solve(case_no: u32, nroygbv: &Vec<u16>) -> String
{
    let mut counts: Counts = nroygbv.iter().skip(1).collect();
    let mut sol = Vec::new();
    let is_ans = primary_color_sol(&mut sol, &mut counts);

    debug!("Solution is {:?}", sol);
    if is_ans
    {
        format!(
            "Case #{}: {}\n",
            case_no,
            format!("{:.2}", sol.iter().format(""))
        )
    }
    else
    {
        format!("Case #{}: IMPOSSIBLE\n", case_no)
    }
}
