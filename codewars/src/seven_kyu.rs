#[allow(non_snake_case)]
fn DNA_strand(dna: &str) -> String {
    return dna
        .chars()
        .map(|c| match c {
            'A' => 'T',
            'T' => 'A',
            'C' => 'G',
            'G' => 'C',
            _ => c,
        })
        .collect();
}
// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

#[test]
fn returns_expected() {
    assert_eq!(DNA_strand("AAAA"), "TTTT");
    assert_eq!(DNA_strand("ATTGC"), "TAACG");
    assert_eq!(DNA_strand("GTAT"), "CATA");
}

fn encode(msg: String, n: i32) -> Vec<i32> {
    let mut digits: Vec<i32> = Vec::new();
    let mut n = n;
    while n != 0 {
        digits.push(n % 10);
        n /= 10;
    }
    digits.reverse();
    return msg
        .as_bytes()
        .iter()
        .enumerate()
        .map(|(i, b)| ((*b as i32) - 'a' as i32 + digits[i % digits.len()] + 1))
        .collect();
}

#[test]
fn fixed_tests() {
    assert_eq!(encode("scout".to_string(), 1939), vec![20, 12, 18, 30, 21]);
    assert_eq!(
        encode("masterpiece".to_string(), 1939),
        vec![14, 10, 22, 29, 6, 27, 19, 18, 6, 12, 8]
    );
}

fn new_avg(arr: &[f64], newavg: f64) -> Option<i32> {
    match newavg * (1 + arr.len()) as f64 - arr.iter().sum::<f64>() {
        n if n >= 0f64 => Some(n.ceil() as i32),
        _ => None,
    }
}

//use std::time::Instant;

fn test_new_avg(arr: &[f64], newavg: f64, exp: Option<i32>) -> () {
    assert_eq!(exp, new_avg(arr, newavg))
}

#[test]
fn new_avg_tests() {
    let a1 = [14.0, 30.0, 5.0, 7.0, 9.0, 11.0, 16.0];
    test_new_avg(&a1, 90.0, Some(628));
    let a2 = [14.0, 30.0, 5.0, 7.0, 9.0, 11.0, 15.0];
    test_new_avg(&a2, 92.0, Some(645));
    let a3 = [14.0, 30.0, 5.0, 7.0, 9.0, 11.0, 15.0];
    test_new_avg(&a3, 2.0, None);
    let a4 = [14000.25, 300.76, 50.56, 70.0, 90.0, 11.0, 150.48, 1200.98];
    test_new_avg(&a4, 4800.0, Some(27326));
}

fn to_leet_speak(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'A' => '@',
            'B' => '8',
            'C' => '(',
            'E' => '3',
            'G' => '6',
            'H' => '#',
            'I' => '!',
            'L' => '1',
            'O' => '0',
            'S' => '$',
            'T' => '7',
            'Z' => '2',
            other => other,
        })
        .collect()
}

#[test]
fn leet() {
    assert_eq!(to_leet_speak("LEET"), "1337");
}

#[test]
fn codewars() {
    assert_eq!(to_leet_speak("CODEWARS"), "(0D3W@R$");
}

#[test]
fn hello_world() {
    assert_eq!(to_leet_speak("HELLO WORLD"), "#3110 W0R1D");
}

#[test]
fn lorem_ipsum() {
    assert_eq!(
        to_leet_speak("LOREM IPSUM DOLOR SIT AMET"),
        "10R3M !P$UM D010R $!7 @M37"
    );
}

#[test]
fn quick_brown_fox() {
    assert_eq!(
        to_leet_speak("THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG"),
        "7#3 QU!(K 8R0WN F0X JUMP$ 0V3R 7#3 1@2Y D06"
    );
}

mod moves_in_squared_strings {
    fn hor_mirror(s: String) -> String {
        s.split('\n').rev().collect::<Vec<_>>().join("\n")
    }
    fn vert_mirror(s: String) -> String {
        s.split("\n")
            .map(|s| s.chars().rev().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }
    // first parameter: dots have to be replaced by function of one variable
    fn oper(f: fn(String) -> String, s: String) -> String {
        f(s)
    }

    fn testing1(s: &str, exp: &str) -> () {
        assert_eq!(oper(hor_mirror, s.to_string()), exp)
    }
    fn testing2(s: &str, exp: &str) -> () {
        assert_eq!(oper(vert_mirror, s.to_string()), exp)
    }

    #[test]
    fn basics_oper() {
        testing1("lVHt\nJVhv\nCSbg\nyeCt", "yeCt\nCSbg\nJVhv\nlVHt");
        testing1("njMK\ndbrZ\nLPKo\ncEYz", "cEYz\nLPKo\ndbrZ\nnjMK");
        testing1("QMxo\ntmFe\nWLUG\nowoq", "owoq\nWLUG\ntmFe\nQMxo");

        testing2(
            "hSgdHQ\nHnDMao\nClNNxX\niRvxxH\nbqTVvA\nwvSyRu",
            "QHdgSh\noaMDnH\nXxNNlC\nHxxvRi\nAvVTqb\nuRySvw",
        );
        testing2(
            "IzOTWE\nkkbeCM\nWuzZxM\nvDddJw\njiJyHF\nPVHfSx",
            "EWTOzI\nMCebkk\nMxZzuW\nwJddDv\nFHyJij\nxSfHVP",
        );
        testing2("cuQW\nxOuD\nfZwp\neqFx", "WQuc\nDuOx\npwZf\nxFqe");
    }
}
mod moves_in_squared_strings_2 {
    fn repeat(s: &str, n: usize) -> String {
        ::std::iter::repeat(s).take(n).collect()
    }
    fn rot(s: &str) -> String {
        s.chars().rev().collect()
    }
    fn selfie_and_rot(s: &str) -> String {
        s.split('\n')
            .map(|s| s.to_string() + &repeat(".", s.chars().count()))
            .collect::<Vec<_>>()
            .join("\n")
            + "\n"
            + &rot(s)
                .split('\n')
                .map(|s| repeat(".", s.chars().count()) + &s)
                .collect::<Vec<_>>()
                .join("\n")
    }

    // first parameter: dots have to be replaced by function of one variable
    fn oper(f: fn(&str) -> String, s: &str) -> String {
        f(s)
    }

    fn testing1(s: &str, exp: &str) -> () {
        assert_eq!(oper(rot, s), exp.to_string())
    }

    fn testing2(s: &str, exp: &str) -> () {
        assert_eq!(oper(selfie_and_rot, s), exp.to_string())
    }

    #[test]
    fn basics_oper() {
        testing1(
            "fijuoo\nCqYVct\nDrPmMJ\nerfpBA\nkWjFUG\nCVUfyL",
            "LyfUVC\nGUFjWk\nABpfre\nJMmPrD\ntcVYqC\nooujif",
        );
        testing1("rkKv\ncofM\nzXkh\nflCB", "BClf\nhkXz\nMfoc\nvKkr");

        testing2(
            "xZBV\njsbS\nJcpN\nfVnP",
            "xZBV....\njsbS....\nJcpN....\nfVnP....\n....PnVf\n....NpcJ\n....Sbsj\n....VBZx",
        );
        testing2(
            "uLcq\nJkuL\nYirX\nnwMB",
            "uLcq....\nJkuL....\nYirX....\nnwMB....\n....BMwn\n....XriY\n....LukJ\n....qcLu",
        );
    }
}
