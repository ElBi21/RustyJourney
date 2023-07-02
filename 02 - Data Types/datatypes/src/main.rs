const WEIRD_INTEGER: i32 = 46;

fn main() {
    /// In Rust there are also constants. Constants are immutable variables, and they can't be modified
    /// under ANY circumstance. We declare a constant by using the `const` keyword. Each constant must
    /// have its type annotated (so in this case we say that [`WEIRD_NUMBER`] is a constant of type
    /// signed 32-bit wide integer. Usually constants are saved with uppercase names. Constants can
    /// be used globally
    const WEIRD_NUMBER: i32 = 43;

    let my_result: i32 = WEIRD_INTEGER + WEIRD_NUMBER;

    /// Now, we saw previously that it's possible to do shadowing on a variable, but that shadowing
    /// can be extended: we can make an inner scope with `{...}` and make shadowing last only inside
    /// such scope. After the scope ends, the shadowing will end as well. Let us make an example:

    println!("First, there was {my_result}");

    let my_result: i32 = 40;

    println!("But also {my_result} came along");

    {
        let my_result = my_result * 2;
        println!("Then {my_result} came");
    }

    println!("At the end, {my_result} returned");

    /// Notice how [`my_result`] is an immutable variable. In fact, we never modified [`my_result`],
    /// we only shadowed and replaced temporarily with another variable. It's important to keep the
    /// `let` keyword though, since without it we would try to reassign the variable and get a
    /// compile error. With shadowing we can also change the variable's type, while with the
    /// reassignment we can't. For instance, let us consider the following piece of code:
    let some_characters: &str = "Hey there! How you doing?";
    let some_characters: usize = some_characters.len();

    println!("There are {some_characters} characters");

    // We can't instead do the following
    /*
    let mut a_string: &str = "Hey there! I'm good, thank you!";
    a_string = a_string.len();
    */

    /// Each variable has a type in Rust, and at compile time the compiler has to know the type of
    /// each variable. It can understand by how a certain variable is used its type, but it's better
    /// to specify it. In Rust there are **scalar types**: they represent a single value, and there
    /// are four built-in types:
    ///  - integers;
    ///  - floating point numbers;
    ///  - booleans;
    ///  - characters.
    /// We already talked about some integer types in the first package. Floating points are
    /// expressed with `f32` or `f64`, and are formatted as `IEEE 754` numbers.

    let cute_floating_number: f32 = 46.432;

    /// Rust also supports arithmetic operations. There are 5 built-in operations: addition,
    /// subtraction, multiplication, division and remainder. The division can act both as a normal
    /// division and as truncation.

    let w: i32 = 3;
    let x: i32 = 40;
    let y: i32 = 36;
    let z: i32 = -4;

    let an_addition: i32 = x + y;
    let a_subtraction: i32 = x - y;
    let a_multiplication: i32 = x * y;
    let a_division: i32 = x / y;
    let a_truncation: i32 = z / w;
    let a_remainder: i32 = x % y;

    println!("Addition: {an_addition}\n\
              Subtraction: {a_subtraction}\n\
              Multiplication: {a_multiplication}\n\
              Division: {a_division}\n\
              Truncation: {a_truncation}\n\
              Remainder: {a_remainder}");

    /// Booleans are either `true` or `false`

    let t: bool = true;
    let f: bool = false;

    /// Characters are also a built-in type in Rust. There is a difference between string and
    /// character: the character is a 4 bytes large value, thus it can represent more than an ASCII
    /// character (in the sense that they support Unicode characters, but still each variable of
    /// type `char` holds only one character). They are written as follows (notice the single quote
    /// mark: for characters we should use single quote marks, while for strings we use double
    /// quote marks):

    let my_char: char = 'C';


}