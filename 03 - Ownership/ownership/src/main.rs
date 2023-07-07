/// When talking about **ownership**, we usually mean a set of rules that manage the way Rust
/// handles memory. Rust has no garbage collector (so a runtime that checks for unused data in the
/// memory and deletes it), but it uses a system of ownership in order to efficiently manage the
/// memory. Such rules must not be violated, otherwise the compiler will stop. These rules do not
/// impact on the running program.
///
/// Rust takes advantage of both **Stack** and **Heap** memory. The stack memory follows a rule
/// called **Last In, First Out** (**LIFO**), and each data that gets pushed into the stack **must**
/// have a known size. If we know that the data size will change, then we have to store such data in
/// the **heap**. The heap memory, compared on how the stack works, is less organized and seems kind
/// of arbitrary: whenever we have to store something in it, the memory allocator will find for a
/// large enough spot in the heap, returning then a pointer to the location in memory where the data
/// was stored.
///
/// Storing data in the stack is faster, as well as reading, while on the heap it's the opposite.
///
/// When a function gets executed, the function's values, parameters and local variables get all
/// pushed into the stack and, when it ends, the values get popped from the stack.
///
/// There are 3 rules for the ownership:
///  - Each value has an **owner**;
///  - Only one owner at a time can exist;
///  - When the owner goes out of scope, then the value is dropped.

fn main() {
    {
        /* We'll now delve inside the concept of variable scope, and we'll use a_string as an
         * example. At this point, before declaring it, a_string is unavailable: if we try to use it
         * before it gets declared, the compiler will return an error.
         */

        let mut a_string: &str = "Hello World!";        // From this point a_string is available.
                                                        // We do some things with a_string...

        println!("Before changing: {:?}", a_string);
        a_string = "Hey there";
        println!("Inside the scope: {:?}", a_string);
    }
    /* After the scope, a_string is not usable anymore. If we try for instance to un-comment the
     * line below, we'll get a compile error.
     */

    // println!("{:?}", a_string)

    /* In order to illustrate the concept of ownership, we want to use the String type. We don't use
     * other types, like integers or floating point numbers, because they all have fixed length,
     * while Strings have different length and thus are a perfect example to illustrate what Rust
     * does when it saves a value in the heap, rather than in the stack. Strings of type String are
     * different from string literals (the ones of type &str), since this string here can be
     * mutable, while string literals are immutable. Look at this example:
     */

    let mut another_string: String = String::from("Hello World! Again!");

    another_string.push_str(" And again...");   // This method allows to enlarge the string,
                                                      // and it works by appending the string
                                                      // literal at the end of the original string

    println!("{:?}", another_string);

    /* Strings literal are useful because we know always at compile time their size, while this
     * doesn't happen with Strings, since they are mutable. We can't reserve an indefinite space in
     * the memory for a string that "might" change. In order to implement it, we need to reserve
     * some space in the memory, where the size is not known at compile time, and this size must be
     * big enough to hold all the contents of a possibly mutated string.
     *
    */
}
