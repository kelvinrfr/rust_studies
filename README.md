# Study on some rust

I'm using the rust book on this repository: https://doc.rust-lang.org/book/


# Chapters 1,2

Hello world. Rust apparently can run standalone using a cli tool but it can also be compiled using cargo. Some examples are:

Using raw Rust
```rust
fn main() {
    println!("Hello, world!");
}
```
```sh
rustc main.rs
```

Using cargo (package manager)
```sh
cargo new hello_world
cd hello_world
cargo build
./target/debug/hello_world
# Hello, world!
```