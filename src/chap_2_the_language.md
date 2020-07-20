# The Language 🔣

- [**Borrow checker**](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html) (The unique feature of Rust)
- Compiled, statically/strongly typed
- NO:
  - garbage collector
  - `null`, `nil` or `undefined`
  - coercion of types
- Similar syntax to Scala and Swift
- Amazing [documentation](https://www.rust-lang.org/learn)

## [Primitives](https://doc.rust-lang.org/rust-by-example/primitives.html)

The syntax is quite normal, nothing crazy.

```rust
// Each rust program must have a main function
fn main() {
    const LANGUAGE: &str = "en";

    //must use `mut` to make mutable
    let mut five: u32 = 5;
    five = 6;

    // immutable here!
    let four_ints: [i32; 4] = [1, 2, 3, 4];
    let stringy_string = "Hello"; // &str type is inferred here

    let coord = (0.0, 0.0); // simple tuple
    let (x,y) = coord; // desctructuring of the above to x,y var

    // our first encounter with a `macro` note the `!`
    println!("{}", five);
    println!("coord: {:?}", coord);
    println!("x: {}, y: {}", x, y);
}
```

## [Controlling Program Flow](https://doc.rust-lang.org/rust-by-example/flow_control.html)

```rust
# fn main(){
for i in 1..10 {
    if i == 5 {
        println!("{} is my favourite number", i);
    } else {
        println!("{}", i);
    }
}

let mut j = 0;

while j < 10 {
    // The powerful pattern matcher syntax (more on this later)
    match j {
        0..=2 => println!("{} is in the range 0..=2", j),
        _ => println!("We don't care about these")
    }
    // Rust doesn't support post/pre-increment operators eg. j++ or ++j
    j+=1;
}

// infinite loop
/*
loop {}
*/
#}
```

## [Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html) and Getting [Functional](https://doc.rust-lang.org/book/ch13-00-functional-features.html)

```rust
// importing standard libs `::` is used as path separator
use std::fs;
use std::io::Error;

// function taking a string reference (&) ie. slice returning new String
fn loud_string(s: &str) -> String {
    let mut loud_string_mut = String::from(s);
    loud_string_mut.push('!');
    return loud_string_mut; // explicit return
}

fn main(){
    let loud_hello = loud_string("Hello");

    // `expect` is simple way to handle errors and panic
    let dir_files = fs::read_dir(".").expect("A list of files"); 

    // getting a bit functional
    dir_files.for_each(| files | { // `| |` signifies a closure/lambda function
        files.map(| f | {          // another closure
            println!("{:?}",f.path())
        });
    });

    // named closure the similar to `let x = (i) => i + 1;` in JS
    let closure  = | i: i32 | i + 1;

    // filter function
    fn is_even(integer: i32) -> bool {
        integer % 2 == 0
    }

    let even_nums: Vec<i32> = (0..)
        .take_while(|&i| i < 10)        // take below 10
        .filter(|&num| is_even(num))    // filter with our "is_even function"
        .collect();                     // collect into a Vector of i32's

    println!("{:?}", even_nums);
}
```

## [Custom Types](https://doc.rust-lang.org/rust-by-example/custom_types.html) and Building Business Logic

Rust doesn't have `classes` but lightweight `structs` and `enums` provide constructs for object orientated style

```rust
#[derive(Debug)] //deriving a debug trait (allows us to println! on this)
struct Person {
    name: String,
    age: i32,
    height: f32,
    nationality: Nationality
}

#[derive(Debug)]
enum Nationality {
    Australian,
    British,
    None
}

// Define an implementation on our type
impl Person {
    fn build(name: &str, age: i32, height: f32, nationality: Nationality) -> Person {
        Person {
            name: name.to_string(),
            age,
            height,
            nationality
        }
    }
}

#fn main() {
// constructing a custom person type (Rust doesn't have constructors)
let p = Person {
    name: String::from("John"),
    age: 30,
    height: 1.80,
    nationality: Nationality::Australian, 
};

// constructing using our implemented build() function
let p1 = Person::build("John", 30, 1.80, Nationality::Australian);

println!("{:?}", p1);
#}
```

## The Power of [Macros!](https://doc.rust-lang.org/rust-by-example/macros.html)

```rust
# extern crate serde_json;
# extern crate rand;
use serde_json::*; // JSON lib crate
use rand::Rng;     // random number generator crate

fn main() {
    // A powerful example of the JSON DSL macro provided by the serde library
    let john = json!({
        "name": "John Doe",
        "age": 43,
        "phones": ["+44 1234567"]
    });

    let full_name = "John Doe";
    let age_last_year = 42;

    // Our random number generator on another thread
    let mut rng = rand::thread_rng();

    // It gets even better
    let john = json!({
        "name": full_name,
        "age": age_last_year + 1,
        "phones": [
            format!("+44 {}", rng.gen_range(1000000, 9999999)),
        ]
    });

    println!("We have JSON: {}", john);
}
```

## [Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html) with Result<T,E>

Rust doesn't have your typical `try` then `catch` but instead makes use of the container type `Result<T,E>`
where `T` is the successful result and `E` is the error. These are wrapped in `Ok` and `Err` enums.

```rust,editable
use std::fs::read_to_string;

fn main(){

    //let result = read_to_string("nothing.txt");
    let result = read_to_string(".gitignore");

    let result = match result {
        Ok(f) => f,
        Err(e) => panic!("Couldn't open the file: {:?}", e)
    };

    println!("File Contents: {}", result);
}
```

## [Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)

Rust has built-in test annotations and assertions

```rust
#[test]
fn test_something() {
    assert_ne!(42, 0);
}
```