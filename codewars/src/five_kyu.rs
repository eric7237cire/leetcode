//Far more succinct solution: https://www.codewars.com/kata/reviews/589c4c614aa12cb8bc001a32/groups/5bc60d8adcb0ac1e02000886

fn prime_factors(n: i64) -> String {
  let mut n = n as u64;
  let mut d = 2;
  let mut mem = std::collections::BTreeMap::new();
  while d <= n {
    if n % d == 0 {
      n /= d;
      let old = mem.entry(d).or_insert(0);
      *old += 1;
    } else {
      d += 1;
    }
  }
  mem.iter().map(|(key, val)| match *val {
    1 => format!("({})", key),
    _ => format!("({}**{})", key, val),
  }).collect::<String>()
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
    testing_prime_factors(17*17*93*677, "(3)(17**2)(31)(677)");
    testing_prime_factors(86240, "(2**5)(5)(7**2)(11)");
    testing_prime_factors(15485867, "(15485867)");
    testing_prime_factors(7537*123863 , "(7537)(123863)");
}

pub fn trystuff()
{
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

    (last, cur, last*cur==prod)
}

fn test_product_fib(prod: u64, exp: (u64, u64, bool)) -> () {
    assert_eq!(product_fib(prod), exp)
}

#[test]
fn basics_product_fib() {
    test_product_fib(4895, (55, 89, true));
    test_product_fib(5895, (89, 144, false));
    test_product_fib(714, (21,34, true));
    test_product_fib(800, (34, 55, false));
    test_product_fib(0, (0, 1, true));
    test_product_fib(1, (1, 1, true));
    test_product_fib(2, (1, 2, true));
}