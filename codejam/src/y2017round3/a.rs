use crate::util::codejam::run_cases;
use indexmap::IndexSet;
use num_integer::binomial;
use permutohedron::LexicalPermutation;
use std::collections::HashMap;
use std::io::Write;

/*
permutations with repeated elements
digit manipulation
recursion
*/
pub fn solve_all_cases()
{
    run_cases(
        &["A-small-practice", "A-large-practice"],
        "y2017round3",
        |reader, buffer| {
            let mut memo = Memo::new();
            let t = reader.read_int();

            for case in 1..=t {
                let G: &str = reader.read_string();

                write!(buffer, "{}", solve(case, &G, &mut memo)).unwrap();
            }
        },
    );
}

struct Memo
{
    map: HashMap<(usize, usize), u32>,
}

impl Memo
{
    fn count_ancestors(&mut self, num: &[u8]) -> u32
    {
        let index = num.iter().fold(0usize, |a, &d| a * 10 + d as usize);
        //debug!("Index is {} for {:?}", index, num);

        if let Some(ans) = self.map.get(&(index, num.len())) {
            debug!("Memoized");
            return *ans;
        }

        let digit_sum = num.iter().sum::<u8>() as usize;
        //An ancestor can only generate a number whose digit sum <= L
        // 4001 would need 11114 which would be too long
        if digit_sum > num.len() {
            return 1;
        }

        //seed permutation
        let mut perm = Vec::new();
        for _ in 0..num.len() - digit_sum {
            perm.push(0);
        }
        for (pos, count) in num.iter().enumerate() {
            for _ in 0..*count {
                perm.push(pos as u8 + 1);
            }
        }
        //debug!("Perm is {:?} ", perm);
        let perm_digit_sum = perm.iter().sum::<u8>() as usize;

        //none can have ancestors, so short circuit and  directly calculate
        if perm_digit_sum > num.len() {
            let mut sum = 1u32;
            let mut digits_remaining = num.len() as u8;
            for &dc in num.iter() {
                sum *= u32::from(binomial(digits_remaining, dc));
                digits_remaining -= dc;
            }
            self.map.insert((index, num.len()), 1 + sum);

            return 1 + sum;
        }

        let mut permutations = IndexSet::new();

        loop {
            permutations.insert(perm.to_vec());
            if !perm.next_permutation() {
                break;
            }
        }

        //debug!("perms are {:?}",  permutations);
        //needed to prevent infinite recursion
        permutations.remove(num);

        let mut sum = 1;
        for p in permutations {
            sum += self.count_ancestors(&p[..]);
        }

        self.map.insert((index, num.len()), sum);

        sum
    }
    fn new() -> Memo
    {
        Memo {
            map: HashMap::new(),
        }
    }
}
fn solve(case_no: u32, G: &str, memo: &mut Memo) -> String
{
    debug!("Solving case {}", case_no);

    let digits = G
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as u8)
        .collect::<Vec<_>>();

    let count = memo.count_ancestors(&digits[..]);

    //debug!("G {:?} {}", digits, G,);

    format!("Case #{}: {}\n", case_no, count)
}
