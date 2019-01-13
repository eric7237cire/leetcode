use super::super::util::input::*;
use permutohedron::LexicalPermutation;
use std::collections::HashMap;
use indexmap::IndexSet;

/*
permutations with repeated elements
digit manipulation
recursion
*/
pub fn solve_all_cases()
{
    let mut reader = InputReader::new();
    let t = reader.read_int();

    let mut memo = Memo::new();
    for case in 1..=t {
        let G = reader.read_string();

        print!("{}", solve(case, &G, &mut memo));
    }
}

struct Memo
{
    map: HashMap<Vec<u8>, u32>,

}

impl  Memo
{
    fn count_ancestors(&mut self, num: &[u8]) -> u32
    {
        //let index = num.iter().fold(0usize, |a, &d| a * 10 + d as usize);
        //debug!("Index is {} for {:?}", index, num);

        if let Some(ans) = self.map.get(num) {
            return *ans;
        }

        let digit_sum = num.iter().sum::<u8>() as usize;
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

        self.map.insert(num.to_vec(), sum);

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
