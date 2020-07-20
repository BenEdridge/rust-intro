# The Compiler in Action

Let's see some error messages!

*All of the following examples will throw a compiler error (for you to fix)*

## Mutability

```rust,editable
fn main(){
  let _i = 42;
  _i+=1;
}
```

## Variable Overflow 

```rust,editable
fn main(){
    let mut _byte: u8 = 0;

    // Fix this loop syntax
    for i in 0..256 {
        _byte = i;
    }
}
```

## Linting

```rust,editable
// Also enabled using `rustc -D nonstandard-style`
#[deny(non_snake_case,unused_extern_crates)]

extern crate semver;

fn main() {
  let camelCase: &str = "I'm a camel";
  println!("{}", camelCase);
}
```

## Even Typos!

```rust,editable
fn main(){
  let original_cost = 100;
  println!("Price is: ${}", orignal_cost);
}
```

