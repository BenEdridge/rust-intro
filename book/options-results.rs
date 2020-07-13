use std::fs::File;

fn main() {
    let f = File::open("test.txt");
    let f1 = File::open("nonexistant.txt");

    match f {
        Ok(file) => println!("File exists with content: {:?} ", file),
        Err(e) => println!("{}", e),
    }

    match f1 {
        Ok(file) => println!("File exists with content: {:?} ", file),
        Err(e) => println!("{}", e),
    }

    let div = divide(6.0, 3.0);

    match div {
        Some(res) => println!("6.0/0.0 = {:?}", res),
        None => println!("Cannot divide by 0"),
    }

    let div0 = divide(6.0, 0.0);

    match div0 {
        Some(res) => println!("6.0/0.0 = {:?}", res),
        None => println!("Cannot divide by 0"),
    }

    // we can also use if let
    if let Result::Ok = f {}
}

// division account for divide by zero
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn check() {
    
    fn cookable_v2(food: Food) -> Option<Food> {
        have_recipe(food).and_then(have_ingredients)
    }
}
