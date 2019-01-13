//code adapted from https://www.geeksforgeeks.org/generate-unique-partitions-of-an-integer/
use std::collections::BTreeSet;
use std::usize;
fn part(n: i64) -> String {
    assert!(n <= 50);
    let mut prod: BTreeSet<i64> = BTreeSet::new();
    let mut p = vec![0; n as usize]; // An array to store a partition
    let mut k: usize = 0; // Index of last element in a partition
    p[k] = n; // Initialize first partition as number itself

    // This loop first prints current partition, then generates next
    // partition. The loop stops when the current partition has all 1s
    loop {
        prod.insert(p.iter().take(k + 1).product());

        // Generate next partition

        // Find the rightmost non-one value in p[]. Also, update the
        // rem_val is set to # of 1s
        let mut rem_val = 0;
        while k != usize::MAX && p[k] == 1 {
            rem_val += p[k];
            k = match k.checked_sub(1) {
                Some(ok) => ok,
                None => usize::MAX,
            }
        }

        // if k < 0, all the values are 1 so there are no more partitions
        if k == usize::MAX {
            break;
        }

        // Decrease the p[k] found above and adjust the rem_val
        p[k] -= 1;
        rem_val += 1;

        // If rem_val is more, then the sorted order is violated.  Divide
        // rem_val in different values of size p[k] and copy these values at
        // different positions after p[k]
        while rem_val > p[k] {
            p[k + 1] = p[k];
            rem_val = rem_val - p[k];
            k += 1;
        }

        // Copy rem_val to next position and increment position
        p[k + 1] = rem_val;
        k += 1;
    }

    let avg: f64 = ((prod.iter().sum::<i64>()) as f64) / (prod.len() as f64);
    let min = prod.iter().next().unwrap();
    let max = *prod.iter().next_back().unwrap();
    let mut it = prod.iter().skip((prod.len() - 1) / 2);
    let mut median: f64 = *it.next().unwrap() as f64;
    if prod.len() % 2 == 0 {
        median += *it.next().unwrap() as f64;
        median /= 2f64;
    }

    format!(
        "Range: {} Average: {:.2} Median: {:.2}",
        max - min,
        avg,
        median
    )
}

fn testequal(ans: &str, sol: &str) {
    assert!(ans == sol, "Expected \"{}\", got \"{}\".", sol, ans);
}

#[test]
fn returns_expected() {
    testequal(&part(1), "Range: 0 Average: 1.00 Median: 1.00");
    testequal(&part(2), "Range: 1 Average: 1.50 Median: 1.50");
    testequal(&part(3), "Range: 2 Average: 2.00 Median: 2.00");
    testequal(&part(4), "Range: 3 Average: 2.50 Median: 2.50");
    testequal(&part(5), "Range: 5 Average: 3.50 Median: 3.50");
}

mod magnet_particles_in_boxes {
    fn doubles(maxk: i32, maxn: i32) -> f64 {
        let mut ans = 0f64;
        for k in 1..1 + maxk as u64 {
            for n in 1..maxn as u64 + 1 {
                let mut term = 1f64 / k as f64;
                for _ in 0..2 * k {
                    term *= 1f64 / (n as f64 + 1f64);
                }
                ans += term;
            }
        }
        ans
    }

    fn assert_fuzzy_equals(actual: f64, expected: f64) {
        let merr = 1.0e-10;
        let inrange = if expected == 0.0 {
            (actual.abs() <= merr)
        } else {
            ((actual - expected).abs() / expected <= merr)
        };
        if inrange == false {
            println!(
                "Expected value must be near: {:e} but was:{:e}",
                expected, actual
            );
        }
        assert_eq!(true, inrange);
    }

    fn dotest(maxk: i32, maxn: i32, exp: f64) -> () {
        assert_fuzzy_equals(doubles(maxk, maxn), exp);
    }

    #[test]
    fn basic_tests_doubles() {
        dotest(1, 10, 0.5580321939764581);
        dotest(10, 1000, 0.6921486500921933);
        dotest(10, 10000, 0.6930471674194457);
    }
}

mod n_linear {
    fn dbl_linear(n: u32) -> u32 {
        let mut u = vec![1u32; 1];
        let mut x = (0usize, 3);
        let mut y = (0usize, 4);

        for _ in 0..n {
            if x.1 <= y.1 {
                u.push(x.1);
                if x.1 == y.1 {
                    y = (y.0 + 1, 3 * u[y.0 + 1] + 1)
                }
                x = (x.0 + 1, 2 * u[x.0 + 1] + 1)
            } else {
                u.push(y.1);
                y = (y.0 + 1, 3 * u[y.0 + 1] + 1)
            }
        }

        return *u.last().unwrap();
    }
    fn n_linear(m: &[u32], n: usize) -> u32 {
        println!("{:?} n={}", m, n);
        let mut m = m.to_vec();
        m.sort();

        let mut u = vec![1u32];
        //tuples are (cur U index to calculate, next u(m) value
        let mut next_values = m
            .iter()
            .enumerate()
            .map(|(m_idx, m_val)| (1 * m_val + 1, m_idx, 0))
            .collect::<Vec<_>>();

        for i in 0..n {
            next_values.sort();
            let nv = next_values[0].0;
            u.push(nv);
            for j in 0..m.len() {
                if next_values[j].0 == nv {
                    let (_, m_idx, u_idx) = next_values[j];
                    next_values[j] = (u[u_idx + 1] * m[m_idx] + 1, m_idx, u_idx + 1);
                } else {
                    break;
                }
            }
        }

        *u.last().unwrap()
    }
    #[test]
    fn pair_test() {
        assert_eq!(n_linear(&[2, 3], 2), 4);
        assert_eq!(n_linear(&[2, 3], 4), 9);
        assert_eq!(n_linear(&[2, 3], 6), 13);
        assert_eq!(n_linear(&[2, 3], 8), 19);
        assert_eq!(n_linear(&[2, 3], 10), 22);
        assert_eq!(n_linear(&[3, 2], 10), 22);
    }

    #[test]
    fn triplet_test() {
        assert_eq!(n_linear(&[5, 7, 8], 10), 64);
        assert_eq!(n_linear(&[5, 7, 8], 11), 65);
        assert_eq!(n_linear(&[5, 7, 8], 11), 65);
        assert_eq!(n_linear(&[5, 7, 8], 12), 73);
        assert_eq!(n_linear(&[5, 7, 8], 13), 156);
        assert_eq!(n_linear(&[5, 7, 8], 14), 206);
    }
}

mod num_partitions {
    use std::cmp::min;

    fn partitions(n: isize) -> isize {
        let n = n as usize;
        let mut dp = vec![vec![0isize; n + 1]; n + 1];
        //dp[n][m] = # of ways to partition n using digits <= m

        //initialize
        for i in 0..n + 1 {
            dp[0][i] = 1;
            dp[1][i] = 1;
        }

        for p in 2..n + 1 {
            //count placing p by itself [p]

            for max_digit in 1..p + 1 {
                //place 1 of max digit
                let remaining = p - max_digit;
                //place bounds on how high this digit can be
                let rem_max_digit = min(remaining, min(p - 1, max_digit));

                // count of placing [p] then the remaining + the previous loop calculation
                dp[p][max_digit] = dp[remaining][rem_max_digit] + dp[p][max_digit - 1];
            }

            println!("For p={} ", p,);
        }

        dp[n][n]
    }

    #[test]
    fn basic_test_01() {
        assert_eq!(partitions(1), 1, "1");
        //2, 1 1
        assert_eq!(partitions(2), 2);
        //3, 21, 111
        assert_eq!(partitions(3), 3);
        //4, 3 1, 2 2, 2 1 1, 1 1 1 1
        assert_eq!(partitions(4), 5);

        //5, 41, 32, 311, 221, 2111, 11111
        assert_eq!(partitions(5), 7);

        //6 51 42  411 33  321 3111 222  2211 21111 11111
        assert_eq!(partitions(6), 11);
    }

    #[test]
    fn basic_test_05() {
        assert_eq!(partitions(5), 7);
    }

    #[test]
    fn basic_test_10() {
        assert_eq!(partitions(10), 42);
    }

    #[test]
    fn basic_test_25() {
        assert_eq!(partitions(25), 1958);
    }

}
