<h1 align="center">Rusty Journey</h1>
<p align="center"><i>My personal journey through Rust</i></p>

---

## I'm sorry... what?
Yeeeeeah, who asked, right? No one, I know. But I wanted to learn Rust,
and I thought of making this repo. I hope it will be helpful for someone.
Each file is commented with (sometimes unnecessary) docstring (I like them
because of the formatting possibilities), so that each step is documented.

This repo is based on the [Rust official book](https://doc.rust-lang.org/book).
Go check it out if something is missing. The book can also be found in the
`Book` folder. It's just a `PDF` version of the site, nothing fancy. Might
be helpful if you want to study offline as well.

## Journey's journal

**Table of contents**
1. [01 - Basics](#01---Basics)
2. [02 - Data types](#02---data-types)
3. [03 - Ownership](#03---ownership)
4. [04 - Structs](#04---structs)
5. [05 - Enumerations](#05---enumerations)
6. [06 - Packages, Crates and Modules](#06---packages-crates-and-modules)
7. [07 - Collections](#07---collections)
8. [Exercises](https://github.com/ElBi21/RustyJourney/tree/main/Exercises/Exercises.md)
    - [E01](https://github.com/ElBi21/RustyJourney/tree/main/Exercises/reverse_str) - `reverse_str` (Difficulty: ⭐)

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
  - `cargo update`: updates all the needed dependencies for a project.

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
        // Some code
    }
    ```
---

## 03 - Ownership

 - Differences between **heap** and **stack** and how variables get saved;
 - The 3 **ownership** rules:
   - Each value has an **owner**;
   - There is only one owner at a time;
   - Whenever the owner gets out of scope, the value is dropped;
 - Variable scoping;
 - Memory and allocation:
   - Variables interaction with `move`;
 - Deep copy with the `clone()` method;
 - Stack-Only Data copies and the `Copy` trait;
 - Functions and ownership;
 - References:
   - **Mutable** and **Immutable** references
```rust
// Immutable reference
{
    let a_string: String = String::from("Hey there");
    let my_ref: & String = a_string;
}

// Mutable reference
{
    let mut another_String = String::from("Hey hey hey!");
    let my_mut_ref: &mut String = &mut another_string;
}
```
 - Dangling pointers;
 - The `Slice` type.

---

## 04 - Structs
 - Creation of a **struct** with the `struct` keyword:
```rust
struct MyStruct {
    a_string: String,
    an_int: usize,
    a_bool: bool,
}

let an_instance: MyStruct = MyStruct {
    a_string = String::from("Hello World"),
    an_int = 21,
    a_bool = true,
};
```
 - Borrowing of single attributes with the syntax `instance.field`;
 - Tuples struct:
```rust
struct Point(i32, i32, i32);

let point_in_quadrant_1: Point = Point(3, 4, 2);

// To select one attribute
println!("{}", point_in_quadrant_1.2);
```
 - Unit-Like structs:
```rust
struct UnitLike;

let a_unit_like_instance: UnitLike = UnitLike;
```
 - Discussion over the clarity of functions (with `tuples` and `structs`);
 - The `Debug` trait and the `dbg!` macro;
 - Methods with the `impl` keyword:
```rust
struct Triangle {
    base: u32,
    height: u32,
    is_rectangle: bool,
}

impl Triangle {
    fn area(&self) -> u32 {
        ( self.base * self.height ) / 2
    }
}
```
 - Encapsulation;
 - The concept of *automatic referencing and deferencing*;
 - Associated functions:
```rust
struct Triangle {
    base: u32,
    height: u32,
};

impl Triangle {
    fn new(base: u32, height: u32) -> Self {
        Self {
            base,
            height,
        }
    }
}

let a_triangle: Triangle = Triangle::new(40, 63);
```
 - Use of the `debug` trait on structs:
```rust
#[derive(Debug)]
struct Triangle {
    base: i32,
    height: i32,
}
```
---

## 05 - Enumerations

 - Creation of an **enumeration** with `enum`:
```rust
enum TransportProtocol {
    UDP,
    TCP,
}

let tcp_protocol: TransportProtocol = TransportProtocol::TCP;
```
 - Specification of types for the variants of the enumeration;
 - Use of `enums` with `structs`:
```rust
enum IPAddrVersion {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IPAddress {
    version: IPAddrVersion,
    address: String,
}
```
 - Use of `impl` with also enumeration types;
 - The "`null`" value, implemented with the `Option<T>` enumeration:
```rust
enum Option<T> {
    Some(T),
    None,
}
```
 - Use of `match` with the variants of the enumerations;
 - Extraction of the values of an enumeration with the `match` statement;
 - Use of the **wildcard** `_` for pointing to unused variants of the enumeration.
---

## 06 - Packages, Crates and Modules

---

## 07 - Collections