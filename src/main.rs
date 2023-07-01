use std::io;
use rand::Rng;
use std::cmp::Ordering;

/// This space here above is the prelude: here you put all the dependencies of the file

/// Here below is the `main` function, which gets executed each time that we run the code.
/// It can take any parameter inside the `()`.

fn main() {
    gameWithoutLoop();
    gameWithLoop();
}

fn gameWithoutLoop () {
    /// It may seem a function, but actually [`println!`] is a macro. We distinguish macros thanks
    /// to the `!` at the end of the name

    println!("Hello World! We'll now play a little game...\nPlease, input your guess: ");

    /// In order to create variables we use the `let` keyword. By itself the variable would be
    /// immutable, but we can make it become mutable with the `mut` keyword.
    ///
    /// Inside this variable we created a new instance of a [`String`] (similarly to `Java`). By
    /// itself, the string is empty

    let randomNumber = rand::thread_rng().gen_range(1..=100);
    let mut guestGuess = String::new();


    /// Here we are using a method from the `io` library. We can also use a method from any library
    /// without importing by doing something among these lines:
    /// ```rust
    /// std::io::stdin
    /// ```
    /// This way we can use a particular method of a library without the need of importing the whole
    /// library. This is pretty helpful for not loading useless code.
    ///
    /// On the line below, we used the `.read_line()` method to actually read from the terminal the
    /// user's input. The input of such method is a mutable reference to a variable that we defined
    /// beforehand. The fact that we are using a reference is given by the `&` keyword. References
    /// are useful because they let you access some data in the memory without the need of copying
    /// it multiple times for nothing. By default, references are immutable, but can be mutable
    /// thanks to, again, the `mut` keyword.
    ///
    /// The `read_line()` method returns a `Result` value. Such value is an `enumeration`
    /// (or `enum`), so a type that can be one of multiple states. Each state is said `variant`.
    /// For instance, `Result`'s variants are `Ok` and `Err`. As we may expect, `Ok` means that
    /// the operation was successful, while `Err` means that something went wrong.
    ///
    /// In `Result`'s variant was `Err`, then `.expect()` would be triggered. `.expect()` is an
    /// exclusive method of the `Err` variant.

    io::stdin()
        .read_line(&mut guestGuess)
        .expect("Geez, I couldn't read it!");

    println!("So, you inserted {guestGuess}, huh? But will it be right?\nThe secret number was {randomNumber}");

    /// If we try to run the program until here, everything will go fine. But once we'll pass to the
    /// `match` part below, we'll get an error. This happens because we are passing a [`String`] to
    /// the [`.cmp()`] method, which actually asks for numbers. There are multiple types of numbers,
    /// such as `u32` (unsigned 32 bits wide number), `i32` (signed 32 bits wide number), `i64`
    /// (the same of `i32` but with 64 bits), and much more.
    ///
    /// In order to convert the string we have to do the following:

    let mut guestGuess: i32 = guestGuess.trim().parse().expect("Hey, that wasn't a number! Insert a number next time, please");

    match guestGuess.cmp(&randomNumber) {
        Ordering::Less => println!("Ew, that's small"),
        Ordering::Equal => println!("YOO! You guessed it!"),
        Ordering::Greater => println!("Oh boy, that's a big number")
    }
}

fn gameWithLoop () {
    println!("Hello World! We'll now play a little game...");
    let randomNumber = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Please, input your guess: ");

        let mut guestGuess = String::new();

        io::stdin()
            .read_line(&mut guestGuess)
            .expect("[ E ] Geez, I couldn't read it!");

        println!("So, you inserted {guestGuess}");

        let guestGuess: i32 = guestGuess.trim().parse().expect("Hey, that wasn't a number! Insert a number next time, please");

        match guestGuess.cmp(&randomNumber) {
            Ordering::Less => println!("Ew, that's small"),
            Ordering::Equal => {
                println!("YOO! You guessed it!");
                break;
            },
            Ordering::Greater => println!("Oh boy, that's a big number")
        }
    }
}