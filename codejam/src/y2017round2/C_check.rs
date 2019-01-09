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
	R: usize,
	C: usize,
	cs: Vec<Vec<char>>
}
fn read() -> Input {
	let (R, C) = readln();
	let cs = readlns(R);
	Input { R, C, cs }
}

type V = usize;

#[derive(Clone, Debug)]
pub struct Graph {
	pub es: Vec<Vec<V>>,
	pub rs: Vec<Vec<V>>
}

impl Graph {
	pub fn new(n: usize) -> Graph {
		Graph { es: vec![vec![]; n], rs: vec![vec![]; n] }
	}
	pub fn add(&mut self, v: V, to: V) {
		self.es[v].push(to);
		self.rs[to].push(v);
	}
	/// Compute (#sccs, [scc_id; n]).
	/// scc_id is topologically sorted, i.e., if scc_id[u] < scc_id[v], there are no edges from v to u.
	pub fn solve(&self) -> (usize, Vec<usize>) {
		let n = self.es.len();
		let mut visit = vec![false; n];
		let mut us = vec![];
		for v in 0..n {
			if !visit[v] {
				dfs(&self.es, &mut visit, &mut |u| us.push(u), v)
			}
		}
		visit = vec![false; n];
		let (mut nscc, mut scc_id) = (0, vec![0; n]);
		for v in us.into_iter().rev() {
			if !visit[v] {
				dfs(&self.rs, &mut visit, &mut |u| scc_id[u] = nscc, v);
				nscc += 1;
			}
		}
		(nscc, scc_id)
	}
}

fn dfs<F>(es: &Vec<Vec<V>>, visit: &mut Vec<bool>, f: &mut F, v: V) where F: FnMut(V) {
	visit[v] = true;
	for &u in &es[v] {
		if !visit[u] {
			dfs(es, visit, f, u)
		}
	}
	f(v)
}


const DX: [i32; 4] = [1, 0, -1, 0];
const DY: [i32; 4] = [0, 1, 0, -1];

fn shoot(cs: &Vec<Vec<char>>, mut x: i32, mut y: i32, mut d: usize) -> Option<Vec<(usize, usize)>> {
	let X = cs.len() as i32;
	let Y = cs[0].len() as i32;
	let mut ps = vec![];
	loop {
		x += DX[d];
		y += DY[d];
		if x < 0 || X <= x || y < 0 || Y <= y || cs[x as usize][y as usize] == '#' { break }
		let c = cs[x as usize][y as usize];
		if c == '.' {
			ps.push((x as usize, y as usize));
		} else if c == '|' || c == '-' {
			return None;
		} else if c == '\\' {
			d = d ^ 1;
		} else if c == '/' {
			d = 3 - d;
		} else {
			assert!(false);
		}
	}
	Some(ps)
}

fn solve(mut it: Input) -> String {
	let mut id = mat![!0; it.R; it.C];
	let mut cand = mat![vec![]; it.R; it.C];
	let mut ps = vec![];
	let mut N = 0;
	for i in 0..it.R {
		for j in 0..it.C {
			if it.cs[i][j] == '|' || it.cs[i][j] == '-' {
				id[i][j] = N;
				ps.push((i, j));
				N += 1;
			}
		}
	}
	let mut g = Graph::new(N * 2);
	for i in 0..N {
		for d in 0..2 {
			let x = ps[i].0 as i32;
			let y = ps[i].1 as i32;
			let a = shoot(&it.cs, x, y, d);
			let b = shoot(&it.cs, x, y, d + 2);
			if a.is_none() || b.is_none() {
				g.add(i * 2 + d, i * 2 + 1 - d);
			} else {
				for (x, y) in a.unwrap() {
					cand[x][y].push(i * 2 + d);
				}
				for (x, y) in b.unwrap() {
					cand[x][y].push(i * 2 + d);
				}
			}
		}
	}
	for x in 0..it.R {
		for y in 0..it.C {
			if it.cs[x][y] == '.' {
				if cand[x][y].len() == 0 {
					return "IMPOSSIBLE".to_string();
				} else if cand[x][y].len() == 1 {
					let v = cand[x][y][0];
					g.add(v ^ 1, v);
				} else {
					assert!(cand[x][y].len() == 2);
					let u = cand[x][y][0];
					let v = cand[x][y][1];
					g.add(u ^ 1, v);
					g.add(v ^ 1, u);
				}
			}
		}
	}
	let (n, scc) = g.solve();
	for i in 0..N {
		if scc[i * 2] == scc[i * 2 + 1] {
			return "IMPOSSIBLE".to_string();
		}
	}
	let mut cs = it.cs.clone();
	for i in 0..N {
		if scc[i * 2] < scc[i * 2 + 1] {
			cs[ps[i].0][ps[i].1] = '-';
		} else {
			cs[ps[i].0][ps[i].1] = '|';
		}
	}
	let mut res = "POSSIBLE\n".to_string();
	for x in 0..it.R {
		let s: String = cs[x].iter().collect();
		res += &s;
		res += "\n";
	}
	return res;
}
