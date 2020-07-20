# Checking Code, Docs and Debugging

## The `linter`, `rust-clippy` and code formatter `rustfmt`

`rustup` gives the possibility to install a linter and code formatter
providing functions similar to `eslint` and `prettier`

Linting by default with `rustc`
[https://doc.rust-lang.org/rustc/lints/groups.html](https://doc.rust-lang.org/rustc/lints/groups.html)

`rustc main.rs -D nonstandard-style`

```
error: variable `camelCase` should have a snake case name
  --> src/main.rs:30:9
   |
30 |     let camelCase = "I'm a camel";
   |         ^^^^^^^^^ help: convert the identifier to snake case: `camel_case`
   |
   = note: `-D non-snake-case` implied by `-D nonstandard-style`
```

## `rustdoc` and `mdbook`

Rust source code supports `///` comment annotations to generate full blown documentation. `mdbook` creates a full book from markdown and code annotations (This book was created in `mdbook`!)

```rust,editable
/// You can have rust code between fences inside the comments
/// If you pass --test to `rustdoc`, it will even test it for you!
/// ```
/// use doc::Person;
/// let person = Person::new("name");
/// ```

```

## Debugging with `gdb` and `lldb`

Many problems with your code will be picked up by the compiler but logic errors won't.

- Rust doesn't provide a debugger itself but debug symbols for use with a native debugger 
- Both VSCode and IntelliJ have plugin support for lldb and gdb

For example in VSCode you could install:

[CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)
or
[Native Debug](https://marketplace.visualstudio.com/items?itemName=webfreak.debug)