#[allow(non_snake_case)]
fn DNA_strand(dna: &str) -> String {
    return dna.chars().map( |c| match c {
        'A' => 'T',
        'T' => 'A',
        'C' => 'G',
        'G' => 'C',
        _ => c
    }).collect();  
}
// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

#[test]
fn returns_expected() {
  assert_eq!(DNA_strand("AAAA"),"TTTT");
  assert_eq!(DNA_strand("ATTGC"),"TAACG");
  assert_eq!(DNA_strand("GTAT"),"CATA");
}

fn encode(msg: String, n: i32) -> Vec<i32> {
    let mut digits: Vec<i32> = Vec::new();
    let mut n = n;
    while n != 0
    {
        digits.push(n%10);
        n /= 10;
    }
    digits.reverse();
    return msg.as_bytes().iter().enumerate().map(|(i,b)| ( (*b as i32)- 'a' as i32+digits[i%digits.len()]+1) ).collect();
}


#[test]
fn fixed_tests() {
    assert_eq!(encode("scout".to_string(), 1939), vec![20, 12, 18, 30, 21]);
    assert_eq!(encode("masterpiece".to_string(), 1939), vec![14, 10, 22, 29, 6, 27, 19, 18, 6, 12, 8]);
}