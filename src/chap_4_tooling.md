# Tooling 🛠️

Below I have compared Rust tools to those in standard NodeJS environment.

## Getting started with [rustup](https://rustup.rs)

The tool-chain installer and go to for cross-compilation, version updates, profile management and plugins.

The `nvm` or `n` version manager of Rust but heap more powerful

```
brew install rustup-init
```
[For Other Methods and GPG verification](https://forge.rust-lang.org/infra/other-installation-methods.html)

eg. Install the nightly version of Rust
```
rustup toolchain install nightly
```

## The package manager [cargo](https://doc.rust-lang.org/cargo/) and package registry [crates.io](https://crates.io/)

`cargo` is the package manager for Rust that pulls in packages from [crates.io](https://crates.io/) and provides many similar functions to `npm`.

A simple registry can be implemented in Git.

`cargo` provides a number of commands for building, cleaning, installing and publishing packages.

Some of the commands we will be using are:

```python
#Create a new cargo binary
cargo init

#Run a binary or example of the local package
cargo run

#Build this package's and its dependencies' documentation
cargo doc

#Run the tests
cargo test
```