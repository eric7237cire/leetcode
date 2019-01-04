use super::super::util::input::read_int_line;
use std::io::stdin;
use std::iter::FromIterator;
use std::slice::Iter;
use std::thread;

pub fn solve_all_cases()
{
    let mut children: Vec<thread::JoinHandle<_>> = vec![];

    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let t = s.trim().parse::<u32>().unwrap();

    for case in 1..=t
    {
        //N, R, O(RY), Y, G(YB), B, and V(RB).
        let input: Vec<u16> = read_int_line();

        children.push(thread::spawn(move || -> String { solve(case, &input) }));
    }

    for child in children
    {
        print!("{}", child.join().unwrap());
    }
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

fn to_index(c: &Colors) -> usize
{
    match c
    {
        Red => 0,
        Orange => 1,
        Yellow => 2,
        Green => 3,
        Blue => 4,
        Violet => 5,
    }
}
// R, O(RY), Y, G(YB), B, and V(RB).
fn is_ok(c1: &Colors, c2: &Colors) -> bool
{
    match c1
    {
        Red => match c2
        {
            Yellow => true,
            Green => true,
            Blue => true,
            _ => false,
        },
        Orange => match c2
        {
            Blue => true,
            _ => false,
        },
        Yellow => match c2
        {
            Red => true,
            Violet => true,
            Blue => true,
            _ => false,
        },
        Green => match c2
        {
            Red => true,
            _ => false,
        },
        Blue => match c2
        {
            Red => true,
            Yellow => true,
            Orange => true,
            _ => false,
        },
        Violet => match c2
        {
            Yellow => true,
            _ => false,
        },
    }
}

use self::Colors::*;
static COLORS: [Colors; 6] = [Red, Orange, Yellow, Green, Blue, Violet];

impl Colors
{
    pub fn iterator() -> Iter<'static, Colors>
    {
        COLORS.into_iter()
    }
}

type Counts = [u16; 7];
struct CountsTuple(Counts);
/*
impl FromIterator<u16> for CountsTuple {
    fn from_iter<I: IntoIterator<Item=u16>>(iter: I) -> Self {
        let mut c : CountsTuple = CountsTuple( [0;7] );
        let mut i = 1;
        for v in iter {
            c.0[i]=v;
            i+=1;
        }

        c
    }

}*/
impl<'a> FromIterator<&'a u16> for CountsTuple
{
    fn from_iter<I: IntoIterator<Item = &'a u16>>(iter: I) -> Self
    {
        let mut c: CountsTuple = CountsTuple([0; 7]);
        let mut i = 1;
        let mut n = 0;
        for v in iter
        {
            c.0[i] = *v;
            i += 1;
            n += *v;
        }
        c.0[0] = n;
        c
    }
}

fn helper(sol: &mut Vec<Colors>, counts: &mut Counts, level: usize) -> bool
{
    let r_val = match counts[0]
    {
        0 => true,
        1 =>
        {
            let remaining_color_index = counts
                .iter()
                .enumerate()
                .skip(1)
                .max_by_key(|&(_, item)| item)
                .unwrap()
                .0;
            let color = &COLORS[remaining_color_index];
            //check both ends
            if is_ok(sol.first().unwrap(), color) && is_ok(sol.last().unwrap(), color)
            {
                sol.push(*color);
                counts[0] -= 1;
                counts[remaining_color_index] -= 1;
                true
            }
            else
            {
                false
            }
        }
        _ => {
            
            let mut found = false;
            for idx in 1..7 {
                if counts[idx] == 0 {
                    continue;
                }
                if !is_ok(sol.last().unwrap(), &COLORS[idx]) {
                    continue;
                }
                sol.push(COLORS[idx]);
                counts[0]-=1;
                counts[idx]-=1;
                let ok = helper(sol, counts, level+1);
                if ok {
                    found = true;
                    break;
                } else {
                    sol.remove(sol.len()-1);
                    counts[0]+=1;
                    counts[idx]+=1;
                }
            }

            found   
        },
    };

    debug!("{} Helper sol: {:?}", " ".repeat(level * 2), sol);

    r_val
}

#[allow(non_snake_case)]
fn solve(case_no: u32, nroygbv: &Vec<u16>) -> String
{
    let mut counts: Counts = nroygbv.iter().skip(1).collect::<CountsTuple>().0;
    let mut sol = Vec::new();
    helper(&mut sol, &mut counts, 0);

    debug!("Solution is {:?}", sol);
    format!("Case #{}: \n", case_no)
}
