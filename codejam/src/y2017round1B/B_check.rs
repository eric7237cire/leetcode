///////////////////////////////////////////// Template /////////////////////////////////////////////
#![allow(non_snake_case, unused)]
use std::io::Write;
use std::collections::*;

macro_rules! debug {
    ($($v: expr),*) => {
    	$(let _ = write!(::std::io::stderr(), "{} = {:?} ", stringify!($v), $v);)*
        let _ = writeln!(::std::io::stderr(), "@ {}:{}", file!(), line!());
    }
}
macro_rules! mat {
	($e:expr) => { $e };
	($e:expr; $d:expr $(; $ds:expr)*) => { vec![mat![$e $(; $ds)*]; $d] };
}
macro_rules! ok {
	($a:ident.$f:ident()$(@$t:ident)*) => {
		$a.$f($($t),*)
	};
	($a:ident.$f:ident($e:expr$(,$es:expr)*)$(@$t:ident)*) => { {
		let t = $e;
		ok!($a.$f($($es),*)$(@$t)*@t)
	} };
}
pub trait FromLn {
	fn fromln(s: &str) -> Self;
}
pub fn readln<T: FromLn>() -> T {
	let mut buf = String::new();
	let _ = ::std::io::stdin().read_line(&mut buf).unwrap();
	T::fromln(buf.trim())
}
pub fn readlns<T: FromLn>(n: usize) -> Vec<T> {
	let mut vs = vec![];
	for _ in 0..n { vs.push(readln()); }
	vs
}
macro_rules! fromln_primitives {
	($($t:ty),*) => { $(
		impl FromLn for $t {
			fn fromln(s: &str) -> $t {
				s.parse().unwrap()
			}
		}
	)* }
}
fromln_primitives!(String, bool, f32, f64, isize, i8, i16, i32, i64, usize, u8, u16, u32, u64);
impl<T> FromLn for Vec<T> where T: FromLn {
	fn fromln(s: &str) -> Vec<T> {
		s.split_whitespace().map(T::fromln).collect()
	}
}
impl FromLn for Vec<char> {
	fn fromln(s: &str) -> Vec<char> {
		s.chars().collect()
	}
}
macro_rules! fromln_tuple {
	($($t:ident),*) => {
		impl<$($t),*> FromLn for ($($t),*) where $($t: FromLn),* {
			fn fromln(s: &str) -> ($($t),*) {
				let mut it = s.split_whitespace();
				let t = ($($t::fromln(it.next().unwrap())),*);
				assert_eq!(it.next(), None);
				t
			}
		}
	}
}
fromln_tuple!(A, B);
fromln_tuple!(A, B, C);
fromln_tuple!(A, B, C, D);
pub trait SetMin {
	fn setmin(&mut self, v: Self) -> bool;
}
impl<T> SetMin for T where T: PartialOrd {
	fn setmin(&mut self, v: T) -> bool {
		*self > v && { * self = v; true }
	}
}
pub trait SetMax {
	fn setmax(&mut self, v: Self) -> bool;
}
impl<T> SetMax for T where T: PartialOrd {
	fn setmax(&mut self, v: T) -> bool {
		*self < v && { * self = v; true }
	}
}
pub fn main() {
	let _ = writeln!(::std::io::stderr(), "----- {} -----", file!());
	let _ = ::std::thread::Builder::new().name("run".to_string()).stack_size(32 * 1024 * 1024).spawn(run).unwrap().join();
}
///////////////////////////////////////////// For GCJ /////////////////////////////////////////////
fn run() {
	let mut c = 0; // caseID (1-indexed)
	let mut p = 1; // #threads
	let mut args = ::std::env::args();
	args.next();
	while let Some(a) = args.next() {
		if a == "-c" { c = args.next().unwrap().parse().unwrap() }
		else if a == "-p" { p = args.next().unwrap().parse().unwrap() }
	}
	unsafe { PRE = Box::into_raw(Box::new(init())); }
	let T = readln();
	let inputs: Vec<Input> = (0..T).map(|_| read()).collect();
	if c > 0 {
		let mut out = solve(inputs[c - 1].clone());
		if !out.ends_with("\n") { out += "\n" }
		print!("Case #{}: {}", c, out);
		return;
	}
	let mut outputs = vec![String::new(); T];
	let (tx, rx) = ::std::sync::mpsc::channel();
	let mut t = 0;
	for s in 0..T {
		let _ = write!(::std::io::stderr(), "\r{} / {} ", s + 1, T);
		while t < T && t - s < p {
			let tx = tx.clone();
			let input = inputs[t].clone();
			::std::thread::Builder::new().name(format!("solve({})", t + 1)).stack_size(32 * 1024 * 1024).spawn(move || {
					let mut out = solve(input);
					if !out.ends_with("\n") { out += "\n" }
					tx.send((t, out)).unwrap()
			}).unwrap();
			t += 1;
		}
		let (case, output) = rx.recv().unwrap();
		outputs[case] = output;
	}
	for t in 0..T {
		print!("Case #{}: {}", t + 1, outputs[t]);
	}
}
macro_rules! print { (_) => () }
macro_rules! println { (_) => () }
static mut PRE: *const Pre = 0 as *const Pre;
fn pre() -> &'static Pre { unsafe { &*PRE } }
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
struct Pre {
}
fn init() -> Pre {
	Pre{}
}
#[derive(Debug, Clone)]
struct Input {
	num: Vec<i32>
}
fn read() -> Input {
	let a: Vec<i32> = readln();
	Input{ num: a[1..].to_vec() }
}

const COLOR: [&'static str; 6] = ["R", "O", "Y", "G", "B", "V"];

fn op(i: usize) -> usize {
	(i + 3) % 6
}

fn find(mut num: Vec<i32>) -> Option<String> {
	let mut ss = vec![vec![]; 6];
	for j in 0..6 {
		for i in 0..num[j] {
			ss[j].push(COLOR[j].to_string());
		}
	}
	for i in 0..3 {
		let x = i * 2 + 1;
		let y = op(x);
		while ss[x].len() > 0 {
			if ss[y].len() < 2 {
				if ss[y].len() == 1 && num.iter().fold(0, |x, y| x + y) == 2 {
					return Some(ss[x].pop().unwrap() + &ss[y].pop().unwrap())
				}
				return None
			}
			let mut a = ss[y].pop().unwrap();
			let b = ss[y].pop().unwrap();
			let c = ss[x].pop().unwrap();
			a += &c;
			a += &b;
			ss[y].push(a);
			num[x] -= 1;
			num[y] -= 1;
		}
	}
	let mut ret = String::new();
	let mut start = 0;
	while num[start] == 0 { start += 2 }
	let s = start;
	loop {
		if num[start] == 0 { return None }
		ret += &ss[start].pop().unwrap();
		num[start] -= 1;
		if num[0] + num[2] + num[4] == 0 { break }
		let (a, b) = ((start + 2) % 6, (start + 4) % 6);
		if num[a] == num[b] {
			if a == s { start = a }
			else { start = b }
		} else if num[a] < num[b] { start = b }
		else { start = a }
	}
	if start == s { return None }
	Some(ret)
}

fn solve(mut it: Input) -> String {
	if let Some(s) = find(it.num.clone()) {
		assert_eq!(s.len() as i32, it.num.iter().fold(0, |x, y| x + y));
		for i in 0..s.len() {
			let a = s.chars().nth(i);
			let b = s.chars().nth((i + 1) % s.len());
			assert!(a != b);
		}
		return s;
	}
	"IMPOSSIBLE".to_string()
}
