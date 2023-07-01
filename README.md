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

## 01 - Basics

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

 - Use of `match` to handle errors, similarly to a `try {} catch {}` block:
```rust
let mut user_input: i32 = match guestGuess.trim().parse() {
  Ok(num) => num,
  Err(_) => {
    println!("Hey, that wasn't a number! Insert a number next time, please"); 
}
```
 - Constants (`const` keyword);
 - Shadowing with and without scopes;
 - Scalar types: types that represent a single value. There are 4 built-in types:
   - integers;
   - floating-point numbers;
   - booleans;
   - characters.

Regarding the `integers`:

| length       | signed  | unsigned |
|--------------|---------|----------|
| 8-bit        | `i8`    | `u8`     |
| 16-bit       | `i16`   | `u16`    |
| 32-bit       | `i32`   | `u32`    |
| 64-bit       | `i64`   | `u64`    |
| architecture | `isize` | `usize`  |

- `architecture` means that the compiler adapts the numbers based
on the architecture of the computer that is running the code.
- Numbers can be written as follows, depending on the base of the number:

| Literals    | Example      |
|-------------|--------------|
| Decimal     | `54_654`     |
| Hexadecimal | `0x4F5D`     |
| Octal       | `0o4523`     |
| Binary      | `0b101_1010` |
| Byte        | `b'A'`       |

- `_` can be used to help visualize the number. The computer doesn't read them;
- Overflows can occur, and if they do occur when building, the application
will transform the number into a **two's complement** number:
for instance, let the situation below: `DOES_OVERFLOW` will be wrapped
and become `1`; similarly, `258`will become `2`, and so on and
so forth...

```rust
const DOES_NOT_OVERFLOW: u32 = 23;
const DOES_OVERFLOW: u32 = 257;
```

 - Floating points (handled as `IEEE 754` numbers, there is `f32` and `f64`); 