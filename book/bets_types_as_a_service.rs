#[derive(Debug)]
enum Single {
    Win,
    Place,
    Lose
}

#[derive(Debug)]
enum Double {
    Win,
    Place,
}

#[derive(Debug)]
enum Triple {
    Treble,
    Trio,
    FirstThree,
}

#[derive(Debug)]
enum BetType {
    Single(Single),
    Two(Double),
    Three(Triple)
}

#[derive(Debug)]
struct Bet {
    name: BetType
}

fn main () {
    let b = Bet{ name: BetType::Single(Single::Win) };
    printBetType(b);
}

fn printBetType(bet: Bet){
    println!("{:?}", bet);
}