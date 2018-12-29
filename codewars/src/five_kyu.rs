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

fn testing(n: i64, exp: &str) -> () {
    assert_eq!(&prime_factors(n), exp)
}

#[test]
fn basics_prime_factors() {
    
    testing(7775460, "(2**2)(3**3)(5)(7)(11**2)(17)");
    testing(17*17*93*677, "(3)(17**2)(31)(677)");
    testing(86240, "(2**5)(5)(7**2)(11)");
    testing(15485867, "(15485867)");
    testing(7537*123863 , "(7537)(123863)");
}