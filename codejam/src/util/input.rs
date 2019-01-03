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
