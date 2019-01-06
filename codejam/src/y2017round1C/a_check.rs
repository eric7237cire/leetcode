use std::io::{self, Stdin};
use std::str::{self, FromStr};
use std::error::Error;
fn solve(sc: &mut Scanner) -> f64 {
    let (n, k): (usize, usize) = (sc.ne(), sc.ne());
    let mut pan = Vec::new();
    for _ in 0..n {
        let (r, h): (f64, f64) = (sc.ne(), sc.ne());
        let m = h * 2.0 * r * std::f64::consts::PI;
        pan.push((r, h, m));
    }
    pan.sort_by(|&a, &b| a.2.partial_cmp(&b.2).unwrap());
    let mut ans = 0.0;
    for i in 0..n {
        let mut r_max = pan[i].0;
        let mut sum = pan[i].2;
        let mut cnt = 1;
        for j in (0..n).rev() {
            if cnt == k {
                break;
            }
            if i == j {
                continue;
            }
            sum += pan[j].2;
            if r_max < pan[j].0 {
                r_max = pan[j].0;
            }
            cnt += 1;
            if cnt == k {
                break;
            }
        }
        sum += r_max * r_max * std::f64::consts::PI;
        if sum > ans {
            ans = sum;
        }
    }
    ans
}
fn exec() {
    let mut sc = Scanner::new();
    let n: usize = sc.ne();
    for i in 0..n {
        println!("Case #{}: {}", i + 1, solve(&mut sc));
    }
}

fn main() {
    const STACK: usize = 16 * 1024 * 1024;
    let _ = std::thread::Builder::new()
        .stack_size(STACK)
        .spawn(|| { exec(); })
        .unwrap()
        .join()
        .unwrap();
}

#[allow(dead_code)]
struct Scanner {
    stdin: Stdin,
    id: usize,
    buf: Vec<u8>,
}

#[allow(dead_code)]
impl Scanner {
    fn new() -> Scanner {
        Scanner {
            stdin: io::stdin(),
            id: 0,
            buf: Vec::new(),
        }
    }
    fn next_line(&mut self) -> Option<String> {
        let mut res = String::new();
        match self.stdin.read_line(&mut res) {
            Ok(0) => None,
            Ok(_) => Some(res),
            Err(why) => panic!("error in read_line: {}", why.description()),
        }
    }
    fn next<T: FromStr>(&mut self) -> Option<T> {
        while self.buf.len() == 0 {
            self.buf = match self.next_line() {
                Some(r) => {
                    self.id = 0;
                    r.trim().as_bytes().to_owned()
                }
                None => return None,
            };
        }
        let l = self.id;
        assert!(self.buf[l] != b' ');
        let n = self.buf.len();
        let mut r = l;
        while r < n && self.buf[r] != b' ' {
            r += 1;
        }
        let res = match str::from_utf8(&self.buf[l..r])
                  .ok()
                  .unwrap()
                  .parse::<T>() {
            Ok(s) => Some(s),
            Err(_) => {
                panic!("parse error, {:?}",
                       String::from_utf8(self.buf[l..r].to_owned()))
            }
        };
        while r < n && self.buf[r] == b' ' {
            r += 1;
        }
        if r == n {
            self.buf.clear();
        } else {
            self.id = r;
        }
        res
    }
    fn ne<T: FromStr>(&mut self) -> T {
        self.next::<T>().unwrap()
    }
}
