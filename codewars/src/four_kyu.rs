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

//More succinct solution here, but iterates through string for each count
//https://www.codewars.com/kata/reviews/58a08dadbf33dfd910000631/groups/5b314e66248a4ebf13001449
use std::collections::HashMap;
fn mix(s1: &str, s2: &str) -> String {
    let str_list = [s1, s2];
    let mut counts : Vec<HashMap<char,usize>> = Vec::new();

    //Initialize char=>char count map
    for _ in 0..2 {        
        counts.push( (b'a'..b'z'+1).map( |cint| (cint as char, 0)).collect::<HashMap<char,_>>() );
    }
    
    //Count the characters
    for (s, count_map) in str_list.iter().zip(counts.iter_mut()) {
        for c in s.chars() {
            if !count_map.contains_key(&c) {
                continue;
            }
            *count_map.get_mut(&c).unwrap() += 1;
        }
    }
    
    //Build up tuple of (# of occurances of character, the character, then the character 1/2/= 
    let mut v : Vec<(&usize, &char, char)> = Vec::new();
    for (count_index,c_map) in counts.iter().enumerate() {
        v.extend(
            c_map.iter()
            .filter( |&(_, total)| *total > 1) //filter out single instances
            .filter( |&(c, total)| counts[1-count_index].get(c).unwrap() <= total) //filter out if less than the other
            .map( |(c, total)| (total, c, if counts[0][c] == counts[1][c] {'='} else { if count_index == 0 {'1'} else {'2'}} ) ) //create tuple
        );
    }

    v.sort_by(|a,b| (b.0, a.2, a.1).cmp( &(a.0, b.2, b.1)));
    v.dedup(); //remove duplicate = entries (since we have one from str1 and str2)
    
    v.iter().map( |tup| format!("{}:{}", tup.2, tup.1.to_string().repeat(*tup.0))).collect::<Vec<String>>().join("/")
}

fn testing_strings_mix(s1: &str, s2: &str, exp: &str) -> () {
    assert_eq!(&mix(s1, s2), exp)
}


#[test]
fn basics_mix() {

    testing_strings_mix("Are they here", "yes, they are here", 
        "2:eeeee/2:yy/=:hh/=:rr");
    testing_strings_mix("looping is fun but dangerous", "less dangerous than coding", 
        "1:ooo/1:uuu/2:sss/=:nnn/1:ii/2:aa/2:dd/2:ee/=:gg");
    testing_strings_mix(" In many languages", " there's a pair of functions", 
        "1:aaa/1:nnn/1:gg/2:ee/2:ff/2:ii/2:oo/2:rr/2:ss/2:tt");
    testing_strings_mix("Lords of the Fallen", "gamekult", "1:ee/1:ll/1:oo");
    testing_strings_mix("codewars", "codewars", "");
    testing_strings_mix("A generation must confront the looming ", "codewarrs", 
        "1:nnnnn/1:ooooo/1:tttt/1:eee/1:gg/1:ii/1:mm/=:rr");

    testing_strings_mix("my&friend&Paul has heavy hats! &",
        "my friend John has many many friends &",
        "2:nnnnn/1:aaaa/1:hhh/2:mmm/2:yyy/2:dd/2:ff/2:ii/2:rr/=:ee/=:ss");

    testing_strings_mix("mmmmm m nnnnn y&friend&Paul has heavy hats! &",
     "my frie n d Joh n has ma n y ma n y frie n ds n&", "1:mmmmmm/=:nnnnnn/1:aaaa/1:hhh/2:yyy/2:dd/2:ff/2:ii/2:rr/=:ee/=:ss");

    testing_strings_mix("Are the kids at home? aaaaa fffff","Yes they are here! aaaaa fffff", "=:aaaaaa/2:eeeee/=:fffff/1:tt/2:rr/=:hh");
    
}