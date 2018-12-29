fn testing(start: u64, stop: u64, exp: Vec<u64>) -> () {
    assert_eq!(backwards_prime(start, stop), exp)
}

#[test]
fn tests_backwards_prime() {

    let a = vec![13, 17, 31, 37, 71, 73, 79, 97];
    testing(1, 100, a);
    let a = vec![13, 17, 31];
    testing(1, 31, a);

}


//Backwards Read Primes
fn backwards_prime(start: u64, stop: u64) -> Vec<u64> {
    let mut ans = Vec::new();

    for n in start..stop+1 {
        if !is_prime(n) {
            continue;
        }
        let r = reverse_number(n);

        if r==n {
            continue;
        }

        if !is_prime(r) {
            continue;
        }

        ans.push(n);
    }
    
    return ans;
}

pub fn is_prime(n: u64) -> bool
{
    let upper_limit = (n as f64).sqrt() as u64 + 1;
    for i in 2u64..upper_limit+1 {
        if n % i == 0 {
            return false;
        }
    }

    return true;
}

pub fn reverse_number(n: u64) -> u64
{
    let mut n = n;
    let mut reverse = 0;
    while n != 0
   {
      reverse = reverse * 10;
      reverse = reverse + n%10;
      n       = n/10;
   }
   return reverse;
}

/*
void generatePrimes( int maxPrime ) 
{
	memset( vbIsPrime, 1, sizeof vbIsPrime); //.assign(maxPrime + 1, true);
	vbIsPrime[0] = false;
	vbIsPrime[1] = false;
	
	primes.clear();

	//Since we are eliminating via prime factors, a factor is at most sqrt(n)
	int upperLimit = static_cast<int>(sqrt(maxPrime));

	for(int i = 2; i <= upperLimit; ++i) {
		if (!vbIsPrime[i]) {
			continue;
		}

		//Loop through all multiples of the prime factor i.  Start with i*i, because the rest
		//were already covered by previous factors.  Ex, i == 7, we start at 49 because 7*2 through 7*6 
		//we already covered by previous prime factors.
		for(int j = i * i; j <= maxPrime; j += i) {
			vbIsPrime[j] = false;
		}
	}

	for(int i = 0; i <= maxPrime; ++i) {
		if (vbIsPrime[i])
			primes.push_back(i);
	}

}
*/