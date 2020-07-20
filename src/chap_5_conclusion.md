# Conclusion

*Something I found on `/r/rust` which sums up my experience so far:*

> "In Rust you exchange some down-the-road complexity for some upfront complexity"

## Where is Rust Useful?

- Critical applications/components
- Realtime applications where memory management and concurrency is key
- In reducing memory leaks, race conditions and other hard to debug problems

## How You Get There?

- Start with critical components
- Components of new critical systems
- Make use of foreign interfaces ie. C,C++ or even WASM

**A potential flow of work I see is:**
1. Proof of concept in JavaScript, Python or another higher level "relaxed language"
2. Verify and Test the POC
3. Implement fully in Rust
4. Profit!

## Potential Issues

- Prototyping is going to be slower with the compiler and strict type checking:
  - Ruby, NodeJS, Python probably more suitable
- Current lack of more niche use cases eg. Machine learning libraries available in Python
- Significant time to learn the borrow checker and Rust memory management paradigm
- Are we going to have a package explosion like `npm`?

## What I didn't cover

There is quite a lot of content I didn't cover in this intro including:

- Lifetimes (related to the memory management)
- Generics
- Managing Strings (Strings are complicated if handled correctly)
- Declarative and Procedural Macros (Meta-programming Goodness)
- `unsafe` (C interop or dangerous memory manual management!)

## Learning More

- [Choose your learning path](https://www.rust-lang.org/learns)
  - [The Rust Book](https://doc.rust-lang.org/book/)
  - [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/)
  - Small exercises: [Rustlings](https://github.com/rust-lang/rustlings/)
  - [The Rust Reference](https://doc.rust-lang.org/stable/reference/)
- [Interactive Rust tour](https://tourofrust.com)

