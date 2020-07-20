#![feature(allow_internal_unstable)]
#![feature(format_args_nl)]
#![feature(print_internals)]

use rayon::prelude::*;
use orion::pwhash;
use serde_json::json;

/// Returns a person with the name given them
///
/// # Arguments
///
/// * `name` - A string slice that holds the name of the person
///
/// # Example
///
/// ```rust,editable
/// // You can have rust code between fences inside the comments
/// // If you pass --test to `rustdoc`, it will even test it for you!
/// use doc::Person;
/// let person = Person::new("name");
/// ```
/// 

// our meta macro we can do cool meta programming below
// macro_rules! log_redact {
//     () => (std::print!("\n"));
//     ($($arg:tt)*) => ({
//         std::io::_print(std::format_args_nl!($($arg)*));
//     })
// }

fn hash_of_hashes(input: Vec<&str>) -> () {
    input.iter() // Rayon is activated here!
        .map(|i| {
            // *i
            let r = pwhash::hash_password("h", 3, 1<<16).expect("To hash");
        })
        .collect();
}

fn main() {

    let camelCase = "I'm a camel";

    let list = vec!(camelCase, "test", "a");
    let result = hash_of_hashes(list);
    println!("hash-chain: {}", result);

    println!("Getting meta with macros!");

    // log_redact!("{:?}", 1);

    let world_status = String::from("The universe is okay");

    // assign using expression
    let world_check = if 1 == 2 { 
        "The universe is okay"
    } else { 
        world_status.as_str()
        //below breaks it
        // world_status
    };

    println!("{}", world_check);


    // another assigment from a loop expression return (Note the ; is removed)
    const SUCCESS: i32 = 10;
    const FAIL: i32 = 11;

    let mut counter = 0;

    let loop_result = loop {
        counter += 1;

        // retry operation here...
        if counter == SUCCESS {
            break true
        } else if counter == FAIL { // Will never reach this.. Rust doesn't catch it yet.
            break false
        }
    };
    println!("Retry operation successful:  {}", loop_result);



    // ITERATORS are nice :)

    let bets = vec!["Trifecta", "Quaddie", "Trio"];
    let winners = ["Vow And Declare", "Cross Counter", "Rekindlingd"];


    let mut bets_mut = bets.clone();

    // See mutable iterators
    // https://doc.rust-lang.org/stable/rust-by-example/flow_control/for.html

    for bet in bets.iter() {
        match bet {
            &"Trifecta" => println!("There is a professional amongst us"),
            _ => println!("Others {}", bet),
        }
    }

    for bet in bets_mut.iter_mut() {
        *bet = match bet {
            &mut "Trifecta" => "Not Available",
            _ => bet,
        }
    }

    for bet in bets_mut.iter() {
        println!("{}", bet);
    }

    for winner in winners.iter() {
        println!("the value is: {}", winner);
    }

    // we can also get functional
    let i: () = winners.iter()
        .map(|f| println!("map res: {}", f))
        .collect();
    

    let s = String::from("hi");
    // owns_string(s); //lose ownership here.
    uses_string_ref(&s);

    // mutable copy of the above...
    let mut s1: String = s.clone();
    uses_string_ref_as_mut(&mut s1);

    // this will error if the function borrows s
    // this won't error if it uses a ref
    // println!("{}", s);

    let ii = 20;
    copys_int(ii);


    // reference scoping

    let ref_to_nothing = dangling();

    let person1 = build_person("ben", "test@example.com", 1);

    let person2 = Person {
        name: String::from("ben1"),
        ..person1 //similar destructuring to NodeJS
    };

    let mut a: Option<i32> = Some(2);
    a = None;


    use serde_json::json;

    let john = json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
        ]
    });

    // Then we get even better

    let full_name = "John Doe";
    let age_last_year = 42;
    
    let john = json!({
        "name": full_name,
        "age": age_last_year + 1,
        "phones": [
            format!("+44 {}", random_phone())
        ]
    });

}

fn dangling() -> String {
    let s = String::from("hi");
    // &s // won't work
    s //create our ref here
}

fn owns_string(string: String) {
    println!("{}", string);

    // if we want to use we must:
    // return String::from(string);
}

fn copys_int(integer: i32){
    println!("{}", integer);
}

fn uses_string_ref(string: &String){
    // Doesn't take ownership so we don't lose it
    // string.push('1');
    println!("{}", string);
}

fn uses_string_ref_as_mut(string: &mut String){
    // Doesn't take ownership so we don't lose it
    println!("Before change: {}", string);
    // string.push('1');
    dbg!(string.push('1')); //dbg macro is useful
    println!("After change: {}", string);

}

// We really wouldn't want to be dealing with ownership like this everytime!
// Rust has as feature called references to allow us to use refs

struct Person {
    name: String,
    email: String,
    age: i32,
    // method vs function
}

enum IpAddr {
    V4(String),
    V6(String)
}

impl IpAddr {
    fn print(&self) {

    }
}


fn route(ip: IpAddr){

}

impl Person {
    fn print(&self) -> String {
        format!("{} : {}", self.name, self.email)
    }

    // Associted function (Not method) since it doesn't make use of self and can be called
    // Person::new()
    fn new() -> Person {
        Person {
            name: String::from(""),
            email: String::from(""),
            age: 0,
        }
    }
}

//tuple struct
struct RGB (u8, u8, u8);

fn build_person(name: &str, email: &str, age: i32) -> Person {
    Person {
        name: String::from(name),
        email: String::from(email),
        age
    }
}