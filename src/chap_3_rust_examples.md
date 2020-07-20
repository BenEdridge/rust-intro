# Rust in Action

### You will see pattern matching a lot!

From the Rust Book:

```rust
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
```

### The WWW 🌏

**A framework for web applications (Rust + WebAssembly) [yew.rs](https://yew.rs/)**

```rust
// This won't run
fn view(&self) -> Html {
    // LOOK another macro like our json! macro before
    html! {
        <div>
            <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
            <p>{ self.value }</p>
        </div>
    }
}
```

**Actor based web framework [Actix](https://actix.rs/)**

```rust
// This won't run
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
```

### Parallel Programming

**Rust natively supports parallel programming but we also have some cool things:**

- [rayon](https://github.com/rayon-rs/rayon) simplifies data parallelism using `iterator` helpers in Rust. This automatically spins up threads on separate CPUs!

The borrow checker, thread safety and highly concurrent nature of Rust allows powerful tools like `Rayon` to simplify the way we think about problems

```rust,editable
extern crate rayon;
extern crate rand;
use rayon::prelude::*;
use std::time::Instant;
use rand::{distributions::Uniform, Rng};

fn main() {

    // Used for our random large vector below
    let mut rng = rand::thread_rng();
    let range = Uniform::new(1, 10);

    let large_vector: Vec<u64> = (0..1_000_000)
        .map(|_| rng.sample(&range))
        .collect();

    // let start = Instant::now();
    // let sum = sum_of_squares(&large_vector);
    // let duration1 = start.elapsed();
    
    // println!("Sum is: {}, Time: {:?}", sum, duration1);
    
    let start = Instant::now();
    let sum = sum_of_squares_parallel(&large_vector); // SPEED!
    let duration2 = start.elapsed();
    println!("Sum is: {}, Time: {:?}", sum, duration2);
}

fn sum_of_squares_parallel(input: &Vec<u64>) -> u64 {
    // -> `par_iter` is a paraller iterator provided by `rayon`
    input.par_iter().map(|&i| i * i).sum()
}

fn sum_of_squares(input: &Vec<u64>) -> u64 {
    input.iter().map(|&i| i * i).sum()
}
```

Should output something like:

```
Sequential Sum of Squares is: 31650045, 47ms
Parallel   Sum of Squares is: 31650045, 13ms
Speedup of: 3.62x
```

# Others
---
## Game Dev

[Amethyst](https://amethyst.rs)  
[ggez](https://ggez.rs)  
[piston](https://www.piston.rs) 

Games Made from the ⬆️  
[playform](https://github.com/bfops/playform)  
[hematite](https://github.com/PistonDevelopers/hematite)


## A new NodeJS? 

**[deno](https://deno.land/) (The new NodeJS) in TypeScript and with a Rust Runtime**

Written by one of the original creators of NodeJS [Ryan Dahl](https://en.wikipedia.org/wiki/Ryan_Dahl)

```typescript
import { serve } from "https://deno.land/std@0.60.0/http/server.ts";
const s = serve({ port: 8000 });
console.log("http://localhost:8000/");
for await (const req of s) {
  req.respond({ body: "Hello World\n" });
}
```

## Operating Systems

A fully bootable operating system entirely in Rust with terminal, web browser, file browser.
[redox OS](https://www.redox-os.org/)
