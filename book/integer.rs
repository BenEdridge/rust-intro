// integer overflow

// https://doc.rust-lang.org/rustc/lints/groups.html

// rustc src/integer.rs -D nonstandard-style


// Woo built in linting!
#![warn(missing_docs)]
#![deny(unused_variables)]
fn main() {

    // unsigned byte overflow caught by compiler in debug mode
    let mut byteVal: u8 = 0;

    for i in 0..256 {
        byteVal = i;
    }
    
    println!("{}", byteVal);

    let heart_eyed_cat = '😻';


    // tuple destructuring
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    // sorry we can't do this as assignment does not returned assigned value
    let j = p = 6;
}

// implicit return for expression
fn fourty_two () -> i32 { 42 }
