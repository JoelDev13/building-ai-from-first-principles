# Index

- [Installation](#installation)
- [Rust Docs](#rust-docs)
- [Hello World](#hello-world)
- [Cargo](#cargo)
- [std::io](#stdio)
- [unwrap()](#unwrap)
- [match](#match)
- [Ordering](#ordering)
- [rand crate](#rand-crate)
- [Structs](#structs)
- [Enum](#enum)



## Installation
- rustup
- rustc --version
---
## Rust Docs

- https://doc.rust-lang.org
- rustup doc

---

## Hello World

```rust
fn main() {
    println!("Hello world");
}
```
- fn:
Is the keyword to declare a function

- main:
Is the name of the main function

- ()
Indicates the function parameters
In this case it's empty. the function doesn't receive arguments

- { ... }
Delimit the function's code block
Everything inside is executed when main is called

- println!
Is a Rust macro (not a normal function)
Used to print text to the console
The ! indicates it's a macro

> [!NOTE]
A macro in Rust is code that generates other code before the program runs, and that's why it uses ! : https://doc.rust-lang.org/book/ch20-05-macros.html 

- "Hello world"
Is a text string
It's what will be displayed on screen

- ;
Indicates the end of the instruction

## Cargo
### Command
```
cargo new project
cargo run
cargo build
```
## std::io
- Allows reading user input
- Example:
```
  use std::io;
```

## unwrap()
- Extracts the hidden value or makes the program explode (crash)

## match
- Switch-like flow control

## Ordering
- Is an [Enum](#enum) (type with options) used to compare values

```rust
use std::cmp::Ordering;

let secret = 10;
let guess = 5;

// cmp() compares and returns an Ordering
match guess.cmp(&secret) {
    Ordering::Less => println!("Too small"),
    Ordering::Greater => println!("Too big"),
    Ordering::Equal => println!("You got it!"),
}
```

## rand crate
- The main library for random numbers

```rust
use rand::Rng; // We import tools from the rand crate

let secret_number = rand::thread_rng().gen_range(1..=100);
```

## Structs
- way to create new data types by grouping others under the same roof

```rust
// We define the structure
struct User {
    name: String,
    age: u8,
    active: bool,
}

// We create an instance (a real user)
let user1 = User {
    name: String::from("Joel"),
    age: 19,
    active: true,
};
```

## Enum
- A data type that can be one of several different options

```rust
enum WebMessage {
    LoadPage,  // No extra data
    ClickButton(i32, i32), //Contains X and Y coordinates
    WriteText(String),  // Contains a text
}

// A message can only be one of the three at a time
let action = WebMessage::WriteText(String::from("Hello Rust"));
```