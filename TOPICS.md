# Journey's journal

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
  - `i64`: signed 64 bits wide numbers.

---

