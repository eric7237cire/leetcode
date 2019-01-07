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
	($a:ident$([$i:expr])*.$f:ident()$(@$t:ident)*) => {
		$a$([$i])*.$f($($t),*)
	};
	($a:ident$([$i:expr])*.$f:ident($e:expr$(,$es:expr)*)$(@$t:ident)*) => { {
		let t = $e;
		ok!($a$([$i])*.$f($($es),*)$(@$t)*@t)
	} };
}
pub trait FromLn {
	fn fromln(s: &str) -> Self;
}
pub fn readln<T: FromLn>() -> T {
	let mut buf = String::new();
	let _ = ::std::io::stdin().read_line(&mut buf).unwrap_or_else(|e| panic!("{}", e));
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
				s.parse().unwrap_or_else(|e| panic!("{}", e))
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
				let t = ($($t::fromln(it.next().unwrap_or_else(|| panic!("input mismatch: illegal number of elements")))),*);
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
	Pre {}
}
#[derive(Debug, Clone)]
struct Input {
	N: usize,
	P: usize,
	G: Vec<usize>
}
fn read() -> Input {
	let (N, P) = readln();
	let G = readln();
	Input { N, P, G }
}

fn solve(mut it: Input) -> String {
	let mut count = vec![0; 4];
	for g in it.G {
		count[g % it.P] += 1;
	}
	let mut dp = mat![-1; count[1] + 1; count[2] + 1; count[3] + 1];
	dp[0][0][0] = 0;
	let mut max = 0;
	for i1 in 0..count[1]+1 {
		for i2 in 0..count[2]+1 {
			for i3 in 0..count[3]+1 {
				if dp[i1][i2][i3] < 0 { continue }
				let left = it.N - count[0] - i1 - i2 - i3;
				max.setmax(dp[i1][i2][i3] + if left > 0 { 1 } else { 0 });
				for j1 in 0..5 {
					for j2 in 0..5 {
						for j3 in 0..5 {
							if (j1 + 2 * j2 + 3 * j3) % it.P == 0 && j1 + j2 + j3 > 0 && i1 + j1 <= count[1] && i2 + j2 <= count[2] && i3 + j3 <= count[3] {
								ok!(dp[i1 + j1][i2 + j2][i3 + j3].setmax(dp[i1][i2][i3] + 1));
							}
						}
					}
				}
			}
		}
	}
	return (count[0] as i32 + max).to_string()
}
