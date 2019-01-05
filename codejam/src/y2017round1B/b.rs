use super::super::util::input::read_int_line;
use super::super::util::log::init_log;
use std::fmt;
use std::io::stdin;
use std::iter::FromIterator;
use std::slice::Iter;
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

impl Colors
{
    pub fn iterator() -> Iter<'static, Colors>
    {
        COLORS.into_iter()
    }
}

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
    fn remaining_color(&self) -> Colors 
    {
        let remaining_color_index = self.count
                .iter()
                .enumerate()
                .max_by_key(|&(_, item)| item)
                .unwrap()
                .0;
        COLORS[remaining_color_index]
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
    let r = helper(&mut sol, &mut counts, 0);
    assert!(r);
}

#[test]
fn test_helper2()
{
    init_log();
    let mut sol: Vec<Colors> = vec![Red, Red];
    let mut counts = Counts::new();
    counts.adj_count(Blue,  1);
    counts.adj_count(Yellow, 1);
    let r = helper(&mut sol, &mut counts, 0);
    assert!(r);
}
#[test]
fn test_helper3()
{
    init_log();
    let mut sol: Vec<Colors> = vec![];
    let mut counts = Counts::new();
    counts.adj_count(Blue,  2);
    counts.adj_count(Yellow, 4);
    counts.adj_count(Red, 2);
    let r = helper(&mut sol, &mut counts, 0);
    assert!(r);
}

fn helper(sol: &mut Vec<Colors>, counts: &mut Counts, level: usize) -> bool
{
    let r_val = match counts.total
    {
        0 => true,
        1 =>
        {
            let remaining_color = counts.remaining_color();
            //check both ends
            if sol.first().unwrap().is_ok(remaining_color) && sol.last().unwrap().is_ok(remaining_color)
            {
                sol.push(remaining_color);
                counts.adj_count(remaining_color, -1);
                true
            }
            else
            {
                false
            }
        }
        _ =>
        {
            if counts.get_count(Red)
                > 1 + counts.get_count(Yellow) + counts.get_count(Blue)
            {
                false
            }
            else if counts.get_count(Yellow)
                > 1 + counts.get_count(Red) + counts.get_count(Blue)
            {
                false
            }
            else if counts.get_count(Blue)
                > 1 + counts.get_count(Yellow) + counts.get_count(Red)
            {
                false
            }
            else
            {
                let mut found = false;
                for idx in 0..6
                {
                    let color = COLORS[idx];
                    if counts.get_count(color) == 0
                    {
                        continue;
                    }
                    if !sol.is_empty() && !sol.last().unwrap().is_ok(color)
                    {
                        continue;
                    }
                    sol.push(color);
                    counts.adj_count(color, -1);
                    let ok = helper(sol, counts, level + 1);
                    if ok
                    {
                        found = true;
                        break;
                    }
                    else
                    {
                        sol.remove(sol.len() - 1);
                        counts.adj_count(color, 1);
                    }
                }

                found
            }
        }
    };

    if counts.total > 0 && sol.len() > 0
    {
        debug!(
            "Level {} Helper sol: {:?}-{:?} size:{} n: {} counts: {:?} ret={}",
            //" ".repeat(level * 2),
            level,
            sol.first().unwrap(),
            sol.last().unwrap(),
            sol.len(),
            counts.total,
            counts.count
                .iter()
                .zip(COLORS.iter())
                .map(|(cnt, col)| format!("{:?}: {}", col, cnt))
                .collect::<Vec<String>>()
                .join("; "),
            r_val
        );
    }

    r_val
}

use itertools::Itertools;

#[allow(non_snake_case)]
fn solve(case_no: u32, nroygbv: &Vec<u16>) -> String
{
    let mut counts: Counts = nroygbv.iter().skip(1).collect();
    let mut sol = Vec::new();
    let is_ans = helper(&mut sol, &mut counts, 0);

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
