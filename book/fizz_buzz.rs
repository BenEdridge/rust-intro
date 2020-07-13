// fizz buzzing 1-100 with Rust matchers
fn main () {
    for i in 1..=100 {
        match (i % 3, i % 5) {
            (0,0) => println!("FizzBuzz"),
            (0,_) => println!("Fizz"),
            (_,0) => println!("Buzz"),
            (_,_) => println!("{}", i)
        }
    }
}

    // (1..101)
    //     .for_each(|x| {
    //             // if x % 5 == 0 && x % 3 == 0 { println!("FizzBuzz")}
    //             // else if x % 3 == 0 { println!("Fizz")}
    //             // else if x % 5 == 0 { println!("Buzz")}
    //             // else { println!("{}", x) }
    //             match (x%3, x%5) {
    //                 (0,0) => println!("FizzBuzz"),
    //                 (0,_) => println!("Fizz"),
    //                 (_,0) => println!("Buzz"),
    //                 (_,_) => println!("{}", x),
    //             }
    //         }
    //     );