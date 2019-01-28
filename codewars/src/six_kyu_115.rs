mod character_frequency_1
{
use std::collections::BTreeMap;

fn letter_frequency(input: & str) -> BTreeMap < char, i32 > {
    let mut r = BTreeMap::new();

    for c in input.chars() {
        let c = c.to_lowercase().next().unwrap();

        if c < 'a' || c > 'z' {
            continue;
        }


        if !r.contains_key(&c) {
            r.insert(c, 0);
        }

        let new_count = r[&c] + 1;
        r.insert( c, new_count);
    }

    r
}

fn letter_frequency2(input: &str) -> BTreeMap<char, i32> {
    input.to_lowercase()
        .chars()
        .filter(|x| x.is_alphabetic())
        .fold(BTreeMap::new(), |mut acc, chr| {
            *acc.entry(chr).or_insert(0) += 1;
            acc
        })
}

    // Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

#[test]
fn simpleword() {
    let answer: BTreeMap<char, i32> =
    [('a', 2),
     ('c', 1),
     ('l', 1),
     ('t', 1),
     ('u', 1)]
     .iter().cloned().collect();

  assert_eq!(letter_frequency("actual"), answer);
}

#[test]
fn sequence() {
    let answer: BTreeMap<char, i32> =
    [('a', 3),
     ('b', 2),
     ('f', 1),
     ('p', 1),
     ('s', 1),
     ('t', 2),
     ('u', 1),
     ('x', 5)]
     .iter().cloned().collect();

  assert_eq!(letter_frequency("AaabBF UttsP xxxxx"), answer);
}

}