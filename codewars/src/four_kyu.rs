use std::collections::{BTreeMap};

// Most borrowed from different kata https://www.codewars.com/kata/reviews/589c4c614aa12cb8bc001a32/groups/5bc60d8adcb0ac1e02000886
fn calculate_prime_factors(n: i64) -> BTreeMap<u64,u64>
{
    let mut n = n.abs() as u64;
    let mut d = 2;
    let mut mem = BTreeMap::new();
    while d <= n {
        if n % d == 0 {
            n /= d;
            let old = mem.entry(d).or_insert(0);
            *old += 1;
        } else {
            d += 1;
        }
    }
    mem
}
//sum by factors
fn sum_of_divided(l: Vec<i64>) -> Vec<(i64, i64)> {
    
    let prime_factors_list = l.iter().map( |n| calculate_prime_factors(*n));

    let mut mem: BTreeMap<u64,i64> = BTreeMap::new();
    for (n, prime_factors) in l.iter().zip(prime_factors_list) {
        for pf in prime_factors.iter() {
            let cur = mem.entry(*pf.0).or_insert(0);
            *cur += *n ;
        }
    }
  
    mem.iter().map(|(fac, sum)| ( (*fac) as i64, *sum )).collect()
}

fn testing_sum_by_factors(l: Vec<i64>, exp: Vec<(i64, i64)>) -> () {
    assert_eq!(sum_of_divided(l), exp)
}

#[test]
fn basics_sum_by_factors() {
    
    testing_sum_by_factors(vec![12, 15], vec![ (2, 12), (3, 27), (5, 15) ]);
    testing_sum_by_factors(vec![15,21,24,30,45], vec![ (2, 54), (3, 135), (5, 90), (7, 21) ]);
    testing_sum_by_factors(vec![15,30, -45], vec![ (2,30), (3,0), (5, 0) ]);
}