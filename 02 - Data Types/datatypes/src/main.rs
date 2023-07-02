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

    /// We can also use tuples and arrays in Rust, which are called compound types. Inside a tuple
    /// or an array we can use whatever types we want. Tuples though have a fixed length, as well
    /// as arrays. For instance:

    let tuple: (i32, &str, char, f32) = (43, "Hey there", 'S', 56.3248);

    /// A tuple (as well as an array) can be unpacked (in Rust this process is called
    /// "destructuring") in multiple variables as in the following:

    let (a, b, c, d) = tuple;

    println!("We got '{b}' and {d}, but also {a} and {c}");

    /// To access an element of the tuple without unpacking we can use the notation
    /// `tuple_variable.index`

    let e: f32 = tuple.3;

    println!("Without unpacking I got {e}");

    /// Arrays are similar to tuples, because they also have a fixed length. Unlike tuples though,
    /// arrays' elements must be of a same common type. The difference between arrays and tuples is
    /// that arrays are saved in the stack, while tuples in the heap. Arrays are useful when we have
    /// a collection of data of the same type and with a fixed length. An array is defined as
    /// follows:

    let an_array: [i32; 4] = [1, 2, 3, 4];
    let another_array: [&str; 5] = ["Hey", "Hi there", "Hi", "Good morning", "Yo"];

    /// We can also do something similar to a list comprehension of Python in Rust with arrays: for
    /// instance, the following array will produce an array of 5 elements, where each element is a 0

    let zero_array: [i32; 5] = [0; 5];

    /// In order to print an array we have to do the following

    println!("{:?}", zero_array);

    /// In order to access an element of the array, we can do as the following:

    let a: &str = another_array[3];

    println!("{a}");

    /// Calling a function...

    let my_value: i32 = new_function(4, 13);
    println!("We got {:?}", my_value);
}

/// In Rust we can create new function via the `fn` keyword. The format to respect is the following:
/// ```rust
/// fn function_name (parameter1: type1, parameter2: type2, [...] ) {
///     // Code...
/// }
/// ```
/// If a function must return a value we have to indicate the type of the result through an arrow
/// like `->`. The syntax then becomes the following:
/// ```rust
/// fn function_name ([...]) -> return_type {
///     // Code...
/// }
/// ```
/// The return value is either the last value explicitly written or the one given by the `return`
/// keyword.

fn new_function (x: i32, y: i32) -> i32 {
    /// In Rust there are **statements** and **expressions**:
    /// - a **statement** is an instruction that doesn't hold any return value;
    /// - an **expression** evaluates an actual value, returning it.
    /// Creating a variable, for instance, is a statement, while summing two numbers is an
    /// expression

    // This is a statement...
    let mut result: i32 = 0;

    {
        // ...and this is an expression
        result = x + y - (x * (y - 2 * x));
    }

    println!("{result}");
    return result;
}
