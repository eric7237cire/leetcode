use super::super::util::input::read_int_line;
use itertools::Itertools;
use std::fmt;
use std::io::stdin;
use std::iter::FromIterator;

/*

problem modeling/understanding

was close, used the official explanation to implement
*/
pub fn solve_all_cases()
{
    //let mut children: Vec<thread::JoinHandle<_>> = vec![];

    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let t = s.trim().parse::<u32>().unwrap();

    for case in 1..=t {
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
        match self {
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
        match self {
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
        match self {
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

impl From<char> for Colors
{
    fn from(item: char) -> Self
    {
        match item {
            'R' => Red,
            'O' => Orange,
            'Y' => Yellow,
            'G' => Green,
            'B' => Blue,
            'V' => Violet,
            _ => panic!("Character not recognized: {}", item),
        }
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

#[derive(Debug, Clone)]
struct Counts
{
    total: u16,
    count: [u16; 6],
}

impl Counts
{
    fn new() -> Counts
    {
        Counts {
            total: 0,
            count: [0; 6],
        }
    }
    fn get_count(&self, c: Colors) -> u16
    {
        self.count[c.to_index()]
    }
    fn add_color(&mut self, c: Colors, v: u16)
    {
        self.count[c.to_index()] += v;
        self.total += v;
    }
    fn num_colors(&self) -> usize
    {
        self.count.iter().filter(|&cnt| *cnt > 0).count()
    }

    fn remove_color(&mut self, c: Colors, v: u16)
    {
        self.count[c.to_index()] -= v;
        self.total -= v;
    }
    fn max_color(&self) -> Colors
    {
        let max_color_index = self
            .count
            .iter()
            .enumerate()
            .max_by_key(|&(_, item)| item)
            .unwrap()
            .0;
        COLORS[max_color_index]
    }

    fn max_color_ok(&self, c1: Colors, c2: Option<Colors>) -> Option<Colors>
    {
        let max_color_index = self
            .count
            .iter()
            .enumerate()
            .filter(|&(_, count)| *count > 0)
            .filter(|&(idx, _)| {
                COLORS[idx].is_ok(c1) && (c2.is_none() || COLORS[idx].is_ok(c2.unwrap()))
            })
            .max_by_key(|&(_, count)| count);

        match max_color_index {
            None => None,
            Some(iv) => Some(COLORS[iv.0]),
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
        for v in iter {
            c.count[i] = *v;
            i += 1;
            n += *v;
        }
        c.total = n;
        c
    }
}

const DOUBLE_COLORS: [Colors; 3] = [Green, Violet, Orange];
const DOUBLE_COLORS_PAIRS: [Colors; 3] = [Red, Yellow, Blue];

fn solution(counts: &mut Counts) -> Option<String>
{
    let counts_check = counts.clone();
    let mut sol = String::new();

    if counts.total == 1 {
        sol.push(counts.max_color().to_char());
        return Some(sol);
    }

    //2 color check

    if counts.num_colors() == 2 {
        let color1 = counts.max_color();
        let color2 = counts.max_color_ok(color1, None);
        if color2.is_none() {
            return None;
        }
        let color2 = color2.unwrap();
        if !color1.is_ok(color2) {
            debug!("Color1 not ok with color2");
            return None;
        }
        let count_1 = counts.get_count(color1);
        let count_2 = counts.get_count(color2);
        if count_1 != count_2 {
            return None;
        }
        for _ in 0..count_1 {
            sol.push(color1.to_char());
            sol.push(color2.to_char());
        }
        return Some(sol);
    }

    //Now make extended color chains
    let mut chains: [String; 3] = [String::new(), String::new(), String::new()];
    //ROYGBV
    for (idx, &db) in DOUBLE_COLORS.iter().enumerate() {
        let db_count = counts.get_count(db);
        debug!("Count of double color {} is {}", db, db_count);
        if db_count == 0 {
            continue;
        }

        let pc = counts.max_color_ok(db, None);
        if pc.is_none() {
            debug!("No primary color available");
            return None;
        }
        let pc = pc.unwrap();
        let pc_count = counts.get_count(pc);

        if pc_count < db_count + 1 {
            debug!("Not enough PC to create a chain for double color");
            return None;
        }

        counts.remove_color(pc, db_count + 1);
        counts.remove_color(db, db_count);
        chains[idx] = pc.to_char().to_string()
            + &(db.to_char().to_string() + &pc.to_char().to_string()).repeat(db_count as usize);
        debug!("Made a chain {}", chains[idx]);
    }

    //now after chains are created, only after, add them back
    for (idx, s) in chains.iter().enumerate() {
        if s.len() <= 0 {
            continue;
        }

        let pc = DOUBLE_COLORS_PAIRS[idx];
        counts.add_color(pc, 1);
    }
    let N = counts.total;
    let color1 = counts.max_color();

    if counts.get_count(color1) > N / 2
    //floor N/2
    {
        debug!(
            "Count of color {} too high {}.  N={}",
            color1,
            counts.get_count(color1),
            N
        );
        return None;
    }

    let color2 = counts.max_color_ok(color1, None).unwrap();
    let color3 = counts.max_color_ok(color1, Some(color2));

    let mut pass1: Vec<Colors> = Vec::new();

    let pass1_size = N / 2 + N % 2;
    for _ in 0..counts.get_count(color1) {
        pass1.push(color1);
        counts.remove_color(color1, 1);
    }
    for _ in 0..pass1_size as usize - pass1.len() {
        pass1.push(color2);
        counts.remove_color(color2, 1);
    }

    let mut pass2: Vec<Colors> = Vec::new();

    for _ in 0..counts.get_count(color2) {
        pass2.push(color2);
        counts.remove_color(color2, 1);
    }

    if let Some(c3) = color3 {
        for _ in 0..counts.get_count(c3) {
            pass2.push(c3);
            counts.remove_color(c3, 1);
        }
    }
    assert_eq!(pass1.len(), pass1_size as usize);
    assert_eq!(
        pass2.len(),
        N as usize - pass1_size as usize,
        "pass 2 wrong size"
    );
    sol.extend(pass1.iter().interleave(pass2.iter()).map(|c| c.to_char()));

    assert_eq!(sol.len(), N as usize);

    for (idx, s) in chains.iter().enumerate() {
        if s.len() <= 0 {
            continue;
        }

        let pc_char = DOUBLE_COLORS_PAIRS[idx].to_char();
        debug!("Replace {}", pc_char);

        sol = sol.replacen(pc_char, s, 1);
    }
    let first_char: char = sol.chars().next().unwrap();
    let last_char: char = sol.chars().last().unwrap();
    assert!(
        Colors::from(first_char).is_ok(Colors::from(last_char)),
        format!(
            "beg {} not ok with end {}.  Sol: {}",
            first_char, last_char, sol
        )
    );

    for (c1, c2) in sol
        .chars()
        .map(|c| Colors::from(c))
        .tuple_windows::<(_, _)>()
    {
        assert!(
            c1.is_ok(c2),
            format!("{} can't be next to {} {:?}", c1, c2, sol)
        );
    }

    for &c in COLORS.iter() {
        let check_amt = counts_check.get_count(c);
        let actual_amt = sol.chars().filter(|&ch| ch == c.to_char()).count() as u16;
        debug!("Checked color {}.  {}=={}", c, check_amt, actual_amt);
        assert_eq!(check_amt, actual_amt);
    }

    Some(sol)
}

fn solve(case_no: u32, nroygbv: &Vec<u16>) -> String
{
    debug!("Solving case {}", case_no);
    let mut counts: Counts = nroygbv.iter().skip(1).collect();
    let ans = solution(&mut counts);

    format!(
        "Case #{}: {}\n",
        case_no,
        match ans {
            Some(ans) => ans,
            _ => "IMPOSSIBLE".to_string(),
        }
    )
}
