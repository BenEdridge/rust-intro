# It's All About the Memory!

- Some languages like `Java`, `Go` and `NodeJS` have automatic collection of unused memory they keep track of references to memory
- Others like `C`, `C++` require manual memory management. This is fast but error prone, a double deallocate is a bug and a missed deallocate is a memory leak

Rust uses a slightly different concept of memory management where the use and `scope` define the `lifetime` of the variable. These variables are kept track of by the owner and the value dropped once the owner is out of scope.

## Scoping

```rust,editable
fn main(){
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward
    }     
                           // this scope is now over, and s is no longer valid
    println!("{}", s);  // This won't print
}
```

## Ownership ie. Moves

Rust keeps track of the owners of memory (That is a variable owns a piece of memory and has a lifetime during the program).
During stages of execution that memory may be freed after use or borrowed during modification.

```rust, editable
fn main(){
    let s: String = String::from("hello");
    println!("{}", s);
    move_and_print(s); // passing `s` by value
    // uncomment below to see the borrow checker in action
    //println!("{}", s);
}

// s is string stored on the heap and `owned` by this function (It has been moved)
fn move_and_print(s: String) {
    println!("{}", s);
}
```

## Borrowing and References

To use data we can pass by reference instead and `borrow` a reference

**Note: Borrow checker at compile time only**

```rust, editable
fn main(){
    let s = String::from("hello");
    println!("{}", s);
    borrow_and_print(&s); // passing `s` by reference
    println!("{}", s);
}

// s is reference to a String stored on the heap and `borrowed` by this function
fn borrow_and_print(s: &String) {
    println!("{}", s);
}
```

## How to Clone/Copy?

```rust,editable
fn main(){
    let s1 = String::from("hello");
    let s2 = s1; // this is a move of s1 -> s2

    println!("{}, world!", s1); // Will throw compilation error
    println!("{}, world!", s2);
}
```

s2 is not a shallow copy or reference but the actual pointer to the previous s1 with a new name.

## Copying Heap Allocated Variables (Strings + more complicated types)

```rust,editable
fn main(){
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{}, world!", s1);
    println!("{}, world!", s2);
}
```

## Copying Stack Data

This only works for primitive data types (integers, bool, floats, char)
integers have a known size and pushed on stack then off when the lifetime is over.

```rust,editable
fn main(){
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}
```