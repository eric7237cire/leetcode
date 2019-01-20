//Far more succinct solution: https://www.codewars.com/kata/reviews/589c4c614aa12cb8bc001a32/groups/5bc60d8adcb0ac1e02000886

fn prime_factors(n: i64) -> String {
    let mut n = n as u64;
    let mut d = 2;
    let mut mem = ::std::collections::BTreeMap::new();
    while d <= n {
        if n % d == 0 {
            n /= d;
            let old = mem.entry(d).or_insert(0);
            *old += 1;
        } else {
            d += 1;
        }
    }
    mem.iter()
        .map(|(key, val)| match *val {
            1 => format!("({})", key),
            _ => format!("({}**{})", key, val),
        })
        .collect::<String>()
}
/*
fn prime_factors(n: i64) -> String {
    let mut n: u64 = n as u64;
    let n2 = (n as f64).sqrt() as u64 + 1;
    let primes = generate_primes(n2);
    let mut factor_count = vec![0; primes.len()];

    let mut i = 0;
    while i < primes.len()
    {
        let p = primes[i];

        if n % p == 0 {
            n /= p;
            factor_count[i] += 1;
        } else {
            i+=1;
        }
    }

    let mut ans:String = "".to_string();
    for (fc,p) in factor_count.iter().zip(primes).filter( |(fc,_)| **fc > 0 ) {
        let fc = *fc;
        if fc == 1 {
            ans += &format!("({})", &p);
        } else {
            ans += &format!("({}**{})", &p, &fc);
        }
    }
    if n > 1 {
        ans += &format!("({})", &n);
    }

    return ans;
}

fn generate_primes( max_prime:u64 ) -> Vec<u64>
{

    //Since we are eliminating via prime factors, a factor is at most sqrt(n)
    let upper_limit:u64 = (max_prime as f64).sqrt() as u64 + 1;

    let mut v_is_prime = vec![true; max_prime as usize +1] ;

    v_is_prime[0] = false;
    v_is_prime[1] = false;

    let mut primes = Vec::new();

    for i in 2..upper_limit+1 {
        if !v_is_prime[i as usize] {
            continue;
        }

        //Loop through all multiples of the prime factor i.  Start with i*i, because the rest
        //were already covered by previous factors.  Ex, i == 7, we start at 49 because 7*2 through 7*6
        //we already covered by previous prime factors.
        let mut j:u64 = i*i;
        while j <= max_prime
        {
            v_is_prime[j as usize] = false;
            j += i;
        }
    }

    for i in 0 as usize..v_is_prime.len() {
        if v_is_prime[i] {
            primes.push(i as u64);
        }
    }

    primes
}
*/
fn testing_prime_factors(n: i64, exp: &str) -> () {
    assert_eq!(&prime_factors(n), exp)
}

#[test]
fn basics_prime_factors() {
    testing_prime_factors(7775460, "(2**2)(3**3)(5)(7)(11**2)(17)");
    testing_prime_factors(17 * 17 * 93 * 677, "(3)(17**2)(31)(677)");
    testing_prime_factors(86240, "(2**5)(5)(7**2)(11)");
    testing_prime_factors(15485867, "(15485867)");
    testing_prime_factors(7537 * 123863, "(7537)(123863)");
}

pub fn trystuff() {
    println!("Ans {:?}", product_fib(4895));
    println!("What?");
}

/*
fn product_fib(prod: u64) -> (u64, u64, bool) {
    let upper_limit = prod as u64;
    let mut fib = vec![0, 1];
    assert_eq!(2, fib.len());
    assert_eq!(1, fib[1]);
    let mut last = 0;
    let mut cur = 1;
    while cur <= upper_limit {
        fib.push( last+cur );
        last = cur;
        cur = *fib.last().unwrap();
    }

    for w in fib.windows(2) {
        let fn0 = w[0];
        let fn1 = w[1];
        let cur_prod = fn0 * fn1;
        if prod == cur_prod {
            return (fn0, fn1, true);
        } else if prod < cur_prod {
            return (fn0, fn1, false);
        }
    }

    (0,0,false)

}*/

fn product_fib(prod: u64) -> (u64, u64, bool) {
    let (mut last, mut cur) = (0, 1);
    while last * cur < prod {
        let next = cur + last;
        last = cur;
        cur = next;
    }

    (last, cur, last * cur == prod)
}

fn test_product_fib(prod: u64, exp: (u64, u64, bool)) -> () {
    assert_eq!(product_fib(prod), exp)
}

#[test]
fn basics_product_fib() {
    test_product_fib(4895, (55, 89, true));
    test_product_fib(5895, (89, 144, false));
    test_product_fib(714, (21, 34, true));
    test_product_fib(800, (34, 55, false));
    test_product_fib(0, (0, 1, true));
    test_product_fib(1, (1, 1, true));
    test_product_fib(2, (1, 2, true));
}

mod best_travel {
    fn helper(t: i32, k: i32, ls: &[i32], cur_sum: i32) -> Option<i32> {
        if t < 0 || ls.len() == 0 {
            return None;
        }
        if k == 1 {
            return match ls.iter().filter(|&&dist| dist <= t).cloned().max() {
                Some(ans) => Some(ans + cur_sum),
                None => None,
            };
        }

        ::std::cmp::max(
            helper(t - ls[0], k - 1, &ls[1..], cur_sum + ls[0]),
            helper(t, k, &ls[1..], cur_sum),
        )
    }

    fn choose_best_sum(t: i32, k: i32, ls: &Vec<i32>) -> i32 {
        // your code
        match helper(t, k, ls, 0) {
            Some(ans) => ans,
            None => -1,
        }
    }

    fn testing(t: i32, k: i32, ls: &Vec<i32>, exp: i32) -> () {
        assert_eq!(choose_best_sum(t, k, ls), exp)
    }

    #[test]
    fn basics_choose_best_sum() {
        let ts = &vec![50, 55, 56, 57, 58];
        testing(163, 3, ts, 163);
        let ts = &vec![50];
        testing(163, 3, ts, -1);
        let ts = &vec![91, 74, 73, 85, 73, 81, 87];
        testing(230, 3, ts, 228);
        testing(331, 2, ts, 178);
    }
}

mod wierd_prime_generator {

    use std::collections::HashSet;
    use  std::mem::swap;

    fn gcd(x: i64, y: i64) -> i64 {
        if y == 0 {
            return x.abs();
        }

        if y > x {
            return gcd(y, x);
        }

        let mut x = x;
        let mut y = y;

        loop {
            x %= y;
            if x == 0 {
                return y.abs();
            }

            swap(&mut x, &mut y)
        }
    }

    fn count_ones(n: i64) -> i64 {
        println!("count_ones {}", n);
        let mut one_count = 0;
        let mut a_last = 7;

        let mut i = 2;
        for _ in 0..n - 1 {
            let a_next = a_last + gcd(i, a_last);
            let g_term = a_next - a_last;
            a_last = a_next;
            i += 1;

            if g_term == 1 {
                one_count += 1
            }
        }
        one_count + 1
    }

    fn max_pn(n: i64) -> i64 {
        println!("Max pn {}", n);
        let mut p: HashSet<i64> = HashSet::new();
        let mut a_last = 7;

        let mut i = 2;
        while p.len() < (n + 1) as usize {
            let a_next = a_last + gcd(i, a_last);
            p.insert(a_next - a_last);
            a_last = a_next;
            i += 1;
        }

        p.iter().max().cloned().unwrap()
    }

    fn an_over_average(n: i64) -> i64 {
        // your code

        let mut an_over: Vec<i64> = Vec::new();
        let mut a_last = 7;

        let mut i = 2;
        while an_over.len() < n as usize {
            let a_cur = a_last + gcd(i, a_last);
            let g_term = a_cur - a_last;

            if g_term != 1 {
                an_over.push(a_cur / i);
            }

            a_last = a_cur;
            i += 1;
        }

        println!("an_over_average {} ", n);
        an_over.iter().sum::<i64>() / n
    }

    fn testing1(n: i64, exp: i64) -> () {
        assert_eq!(count_ones(n), exp)
    }
    fn testing2(n: i64, exp: i64) -> () {
        assert_eq!(max_pn(n), exp)
    }
    fn testing3(n: i64, exp: i64) -> () {
        assert_eq!(an_over_average(n), exp)
    }

    #[test]
    fn returns_expected() {
        testing1(1, 1);
        testing1(10, 8);
        testing1(100, 90);

        testing2(1, 5);
        testing2(5, 47);
        testing2(7, 101);

        testing3(1, 3);
        testing3(5, 3);
    }
}
