use std::io::stdin;

fn main()
{
    //handle input / output
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let card: Vec<_> = s.split_whitespace().collect();
    let card = card[0].clone().chars().collect::<Vec<_>>();
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let deck: Vec<_> = s.split_whitespace().collect();
    let deck = deck.iter().map( |c| c.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    for dc in deck {
        if dc[0] == card[0] || dc[1] == card[1] {
            println!("YES");
            return;
        }
    }

    println!("NO");
}