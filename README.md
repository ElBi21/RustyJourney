<h1 align="center">Rusty Journey</h1>
<p align="center"><i>My personal journey through Rust</i></p>

---

## I'm sorry... what?
Yeeeeeah, who asked, right? No one, I know. But I wanted to learn Rust,
and I thought of making this repo. I hope it will be helpful for someone.

## Journey's journal

**Table of contents**
1. [01 - Basics](#01---Basics)
2. [02 - Data types](#02---data-types)

---

### 01 - Basics

**Topics covered**:
- Variables (mutability with `mut`, immutability);
- Functions and Macros (such as `println!`);
```rust
fn main () {
    println!("Hello world!");
    other_function();
}
```
- Input from user (with the `std::io` library);
```rust
use std::io;

fn main () {
    io::stdin()
        .read_line(&mut my_variable)
        .expect("[ ERROR ] Geez, I couldn't read it!");
}
```
- References with the `&` keyword;
- The `match` statement;
- The `loop` statement
```rust
fn main () {
  let mut i: i32 = 10;
  loop {
    i = i + 1;
  }
}
```
- Types, that are specified like the following:
```rust
let my_string: String = String::new();
let my_number: i32 = 21;
```
- Numbers and their types:
  - `u32`: unsigned 32 bits wide numbers;
  - `i32`: signed 32 bits wide numbers;
  - `u64`: unsigned 64 bits wide numbers;
  - `i64`: signed 64 bits wide numbers;
  - ...and much more!

- *Cargo* commands:
  - `cargo new <project_name>`: create a new project with name `project_name`;
  - `cargo run`: compiles the project and runs it;
  - `cargo build`: creates an executable of the project and runs it;
  - `cargo update`: updates all the needed dependencies for a project;

---

## 02 - Data types