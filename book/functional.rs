// Some of you enjoy functional programming Rust has this too

// Works with the Trait
// std::iter::Iterator

fn main() {
    let horses = [1, 6, 5, 2, 3, 4];
    let dogs = [11, 7, 9, 8, 10, 12];

    // even numbers win more I've been told

    // We can use .cloned() or * dereference to access the values inside the iterator
    ///
    /// let x = 5;
    // let y = &x; //set y to a reference to x
    // assert_eq!(5, x);
    // assert_eq!(5, *y); // dereference y
    let mut vector = horses
        .iter()
        .filter(|h| *h % 2 == 0) //
        .chain(dogs.iter().filter(|d| *d % 2 == 0))
        .collect::<Vec<_>>(); //convert to collection into sorted BTreeSet
                              // .unwrap_or(vec!());

    // https://github.com/rust-lang/rfcs/issues/2731
    // Hmm no inplace sorting as it's a costly function
    let mut slice = vector.as_mut_slice();
    slice.sort();

    println!("{:#?}", slice);

    fn locate_func(a: i32) -> Box<dyn Fn(i32) -> i32> {
        Box::new(move |b| a + b)
    }

    fn make_adder(a: i32) -> impl Fn(i32) -> i32 {
        move |b| a + b
    }

    println!("Locate func {:?}", locate_func(2)(1));

    println!("Adder func {:?}", make_adder(1)(1));

    fn add1(a: i32) -> i32 {
        a + 1
    }

    fn add2(a: i32) -> i32 {
        a + 2
    }

    //https://doc.rust-lang.org/book/ch19-05-advanced-functions-and-closures.html
    fn fnPlusFn(
        f1: fn(i32) -> i32, 
        f2: fn(i32) -> i32, 
        arg: i32
    ) -> i32 {
        f1(arg) + f2(arg)
    }

    println!("Answer is {}", fnPlusFn(add1, add2, 22));
}
