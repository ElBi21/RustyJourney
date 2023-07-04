<h1 align="center">Rusty Journey</h1>
<p align="center"><i>My personal journey through Rust</i></p>

---

## I'm sorry... what?
Yeeeeeah, who asked, right? No one, I know. But I wanted to learn Rust,
and I thought of making this repo. I hope it will be helpful for someone.
Each file is commented with (sometimes unnecessary) docstring (I like them
because of the formatting possibilities), so that each step is documented.

This repo is based on the [Rust official book](https://doc.rust-lang.org/book).
Go check it out if something is missing.

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
 - Operations:
   - Addition: `x + y`;
   - Subtraction: `x - y`;
   - Multiplication: `x * y`;
   - Division: `x / y` (if `x` is negative then we talk about truncation);
   - Remainder: `x % y`;
 - Booleans (`true` and `false`, the type is `bool`);
 - Characters (`char`);
 - Tuples' length is immutable, but can have multiple types:
```rust
let tuple: (i32, f32, char, &str) = (51, 33.54234, 'H', "Wow");

// Accessing to one item
let an_item: f32 = tuple.1;
let another_item: &str = tuple.3;

// Unpacking
let (w, x, y, z) = tuple;
```
 - Arrays are similar to tuples, but all of their elements must be of the
same type:
```rust
let array: [i32; 5] = [2, 54, 3, 6, 7];

// Accessing to one item
let an_item: i32 = array[1];
let another_item: i32 = array[3];
```

 - Functions (with `fn`):
```rust
fn function_name (parameter1: type1, parameter2: type2, [...]) -> return_type {
    // Code goes here...
    
    // If the function has to return something then a value
    // can be returned with the return keyword
    return some_value;
}
```
 - Functions are called based on their names:
```rust
let mut a_number: i32 = 0;
a_number = a_function(a_number, 40);
println!("I got {:?}", a_number);

fn a_function (x: i32, y: i32) -> i32 {
    let result: i32 = x + y;
    return result;
}
```
 - Conditions and control flow
```rust
if condition {
    // Some code...
} else if condition {
    // Some other code...
} else {
    // Other code...
}
```
 - Can also be done inside a variable:
```rust
let my_condition: bool = true;
let a_number: i32 = if my_condition { 4 } else { 5 };
```
- Loops with `loop`, `while` and `for`:
  - `loop`: executes a block until it gets stopped by a `break` keyword.
   It can be labeled and stopped by calling `break` and the label
    ```rust
    let mut counter: i32 = 100;
    
    'my_loop: loop {
       let a_value: i32 = 10;
        
       loop {
           if counter == 50 && a_value == 10 {
               break 'my_loop;
           } else {
               counter -= 1;
           }
       }
    }
    ```
  - `while`: executes a block until the initial condition is not met anymore
    ```rust
    let index: i32 = 100;
    while index >= 0 {
      index -= 1;
    }
    ```
  - `for`: executes the block for each item in the iterable
    ```rust
    let my_array: [i32; 5] = [34, 65, 236, 8675, 343];
    for item in my_array {
        println!("{:?}", item + 2);
    }
    
    // or...
    
    for index in (1..16) {
        // Do something
    }
    
    // We can add the rev() function to reverse the tuple
    
    for index in (0..400).rev() {
    
    }
    ```
    