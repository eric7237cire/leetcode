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

fn mix(s1: &str, s2: &str) -> String {
    use std::collections::HashMap;
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

//use std::collections::{BTreeMap};
use std::collections::BTreeSet;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::usize;
use std::ops::Deref;
//appears to be a generic tuple struct
//from https://www.codewars.com/kata/reviews/58935d5959e016c0c9000011/groups/58a850b72f7e6e7ec9000031
//https://docs.rs/revord/0.0.2/src/revord/lib.rs.html#34
#[derive(Eq, PartialEq)]
struct RevOrd<N>(N);

impl<V> PartialOrd for RevOrd<V> where V: PartialOrd {
  fn partial_cmp(&self, other:&RevOrd<V>) -> Option<Ordering> {
    (other.0).partial_cmp(&self.0)
  }
}
impl<V> Ord for RevOrd<V> where V: Ord {
    fn cmp(&self, other:&RevOrd<V>) -> Ordering {
      other.0.cmp(&self.0)
    }
}

impl<N> Deref for RevOrd<N> {
  type Target = N;
  #[inline]
  fn deref(&self) -> &N {
    &self.0
  }
}

//Idea from https://stackoverflow.com/questions/49360883/dynamic-programming-code-wars-twice-linear-algorithm-times-out
fn dbl_linear(n: u32) -> u32 {
    let mut u = vec![1u32;1];
    let mut x = (0usize, 3);
    let mut y = (0usize, 4);
    
    for _ in 0..n {
        if x.1 <= y.1 {
            u.push(x.1);
            if x.1 == y.1 {
                y = (y.0+1, 3*u[y.0+1]+1)
            }
            x = (x.0+1, 2*u[x.0+1]+1)
        } else {
            u.push(y.1);
            y = (y.0+1, 3*u[y.0+1]+1)
        }        
    }

    return *u.last().unwrap();
}

fn dbl_linear_inefficient(n: u32) -> u32 {
    let mut min_heap = BinaryHeap::new();
    min_heap.push( RevOrd(1) );
    let mut u: BTreeSet<u32> = BTreeSet::new();

    while u.len() < (n+1) as usize {
        //using custom std::ops::Deref deref operator
        let x = *min_heap.pop().unwrap();
        u.insert(x);
        min_heap.push( RevOrd(x*2+1));
        min_heap.push( RevOrd(x*3+1));
    }

    return *u.iter().next_back().unwrap();
}

fn testing_dbl_linear(n: u32, exp: u32) -> () {
    assert_eq!(dbl_linear(n), exp)
}

#[test]
fn basics_dbl_linear() {
    for i in 0..101 {
        testing_dbl_linear(i, dbl_linear_inefficient(i));    
    }
    
    testing_dbl_linear(0, 1);
    testing_dbl_linear(1, 3);
    testing_dbl_linear(3, 7);
    testing_dbl_linear(5, 10);
    testing_dbl_linear(9, 21);
    testing_dbl_linear(10, 22);
    testing_dbl_linear(20, 57);
    testing_dbl_linear(30, 91);
    testing_dbl_linear(50, 175);
    testing_dbl_linear(100, 447);
}

/*
use std::panic;
let result = panic::catch_unwind(|| {
        let u = usize::MAX + 1;
    });
//https://vmx.cx/cgi-bin/blog/index.cgi/printing-panics-in-rust%3A2017-12-05%3Aen%2CNoise%2CRust
assert!(result.is_err());
    if let Err(panic) = result {
        match panic.downcast::<String>() {
            Ok(panic_msg) => {
                println!("panic happened: {}", panic_msg);
            }
            Err(_) => {
                println!("panic happend: unknown type.");
            }
        }
    }

    return None;
    */
#[allow(dead_code)]    
mod decompose {
    use std::collections::HashMap;

    struct Helper {
        //memo[5] = 2 means 5 can be decomposed and 2^2 is the max/right most element in decomposition
        memo: HashMap<usize, Option<usize>>,
        squares : Vec<usize>
    }
    impl Helper {
        fn decompose_helper(
            &mut self,
            n2: usize,
            level: usize 
            )-> Option<usize> {
            
            if self.memo.contains_key(&n2) {
                return self.memo[&n2];
            }

            

            let mut max_k = match self.squares.binary_search(&n2) {
                Ok(n) => n,
                Err(n) => n-1
            };            
            if level == 0 {
                max_k -= 1;
            }
            //print!("{}", " ".repeat(level));
            //println!("Starting n2={} max k={}", n2, max_k);

            for k in (1..max_k+1).rev() {
                //Check feasability
                /*let max_sum = ( k * (k+1) * (2*k+1) ) / 6;
                if max_sum < n2 {
                    //print!("{}", " ".repeat(level));
                    //println!("For n2={}, k={} max sum is {} breaking", n2, k, max_sum);
                    break;
                }*/
                /*if k*k > n2 {
                    break;
                }*/
                //print!("{}", " ".repeat(level));
                //println!("n2={} k={} max_sum={} k*k={}", n2, k, max_sum, k*k);
                assert!(n2 >= k*k);
                let resp_opt = self.decompose_helper(n2-k*k, 4+level);
                //print!("{}", " ".repeat(level));
                //println!("For n2={} trying k={} resp={:?}", n2, k, resp_opt);
                if let Some(resp) = resp_opt {
                    if resp >= k {
                        continue;
                    }
                    //print!("{}", " ".repeat(level));
                    //println!("For {} ans is k={:?}", n2, k);
                    self.memo.insert(n2, Some(k));
                    return Some(k);
                }
            }
            //print!("{}", " ".repeat(level));
            //println!("For n2={}, max_k={} returning no solution", n2, max_k);
            self.memo.insert(n2, None);
            return None;
        }
    }

    fn decompose(n: i64) -> Option<Vec<i64>> {
        let n = n as usize;
        let mut helper = Helper {
            memo: HashMap::new(),
            squares : (0..n+1).map(|s| s*s).collect()
        };
        helper.memo.insert(0, Some(0));
        helper.memo.insert(1, Some(1));
        helper.memo.insert(5, Some(2));

        //println!("Squares: {:?}", helper.squares);

        let next = helper.decompose_helper(n*n,0);

        return match next {
            None => None,
            Some(next_val) => {
                let mut ans : Vec<i64> = Vec::new();
                let mut next_val  = next_val;
                let mut remaining = n*n - next_val*next_val;
                ans.push(next_val as i64);
                while remaining > 0 {
                    next_val = helper.memo[&remaining].unwrap();
                    ans.push(next_val as i64);
                    remaining -= next_val * next_val;
                }
                ans.reverse();
                Some(ans)
            }
        };   
    }

    fn decompose_also_too_slow(n: i64) -> Option<Vec<i64>> {
        
        let squares : Vec<i64> = (1i64..n).map(|s| s*s).collect();
        let indexes : Vec<i64> = (1..n).collect();
        
        println!("Squares: {:?} len {}", squares, squares.len());

        //this can be 1<<50 which is huge
        for subset in (0..(1<<squares.len())).rev() {
            let mut sum = 0;
            for i in 0usize..n as usize {
                if (1 << i) & subset > 0 {
                    sum += squares[i];
                } 
            }
            //println!("For subset: {:?} sum is {}", subset, sum);
            if sum == n*n {
                let ans : Vec<i64> = (0..n).filter( |i| (1i64 << i) & subset > 0 ).map(|i| indexes[i as usize]).collect();
                return Some(ans);
            }
        }
        

       None     
    }

    

    fn testing_squares_into_squares(n: i64, exp: Option<Vec<i64>>) -> () {
        assert_eq!(decompose(n), exp)
    }

    #[test]
    fn tests_decompose() {
        
        testing_squares_into_squares(50, Some(vec![1,3,5,8,49]));
        testing_squares_into_squares(44, Some(vec![2,3,5,7,43]));
        testing_squares_into_squares(625, Some(vec![2,5,8,34,624]));
        testing_squares_into_squares(5, Some(vec![3,4]));
        testing_squares_into_squares(6, None);
        testing_squares_into_squares(7, Some(vec![2, 3, 6]));
        testing_squares_into_squares(11, Some(vec![1, 2, 4, 10]));
        testing_squares_into_squares(18351, Some(vec![1, 3, 5, 8, 11, 191, 18350]));
        testing_squares_into_squares(38477, Some(vec![4, 8, 12, 277, 38476]));
        testing_squares_into_squares(9226449, Some(vec![1, 2, 3, 8, 13, 75, 4295, 9226448]));

        
    }
}