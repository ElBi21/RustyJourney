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

pub(crate) fn ownership () {
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
     * big enough to hold all the contents of a possibly mutated string. This means that we have to
     * request from the memory allocator some space in the memory and that we have to return (or
     * free) the memory once we're done with the String. The request for some space in memory is
     * done via the call of String::from. But what about the second part? How do we free the memory?
     *
     * In languages where there is a garbage collector we don't have to worry about the memory,
     * while in languages where there is no garbage collector we have to free the memory manually.
     * Each memory allocation should correspond with one memory free. How is it done in Rust?
     *
     * Generally, memory becomes free after the variable that owns that place in memory goes out of
     * scope. For instance, let us consider the following example:
     */

    {
        let a_funny_string: String = String::from("Lol!");

        println!("{}", a_funny_string)
    }

    /* After the scope, the string a_funny_string gets deleted from the memory, and this is done via
     * an implicit function called drop. Such function can't be called explicitly (it can be done
     * through the use of the mem library, but we won't use it for now)
     */

    // Multiple variables can interact between each other. For instance:
    {
        let value_one: i32 = 3243;
        let value_two: i32 = value_one;
    }

    /* In this example we made two variables, we set one as a concrete value, while the other is set
     * as a copy of the first one. All of this happens into the stack, because of the knows, fixed
     * size of the two numbers. This doesn't happen with Strings though. Let's make an example and
     * look why:
     */

    {
        let string_one: String = String::from("Hey");
        let string_two: String = string_one;
    }

    /* We might think that this is the same of what happened with value_one and value_two, but
     * that's not quite right: we can explain it by showing how a string is saved inside the memory:
     *
     *              Stack                       Heap
     *      ┌──────────┬───────┐         ┌───────┬───────┐
     *      │   name   │ value │         │ index │ value │
     *      ├──────────┼───────┤         ├───────┼───────┤
     *      │  pointer │ --------------->│   0   │   H   │
     *      ├──────────┼───────┤         ├───────┼───────┤
     *      │  length  │   3   │         │   1   │   e   │
     *      ├──────────┼───────┤         ├───────┼───────┤
     *      │ capacity │   3   │         │   2   │   y   │
     *      └──────────┴───────┘         └───────┴───────┘
     *
     * Normally a String is saved this way: on the stack are saved a pointer to the heap where the
     * String begins, the length of the string and the capacity (how many bytes the allocator gave
     * to the string). When we assign string_one to string_two, the following happens:
     *
     *       Stack (string_one)                 Heap
     *      ┌──────────┬───────┐         ┌───────┬───────┐
     *      │   name   │ value │         │ index │ value │
     *      ├──────────┼───────┤         ├───────┼───────┤
     *      │  pointer │ --------------->│   0   │   H   │
     *      ├──────────┼───────┤    |    ├───────┼───────┤
     *      │  length  │   3   │    |    │   1   │   e   │
     *      ├──────────┼───────┤    |    ├───────┼───────┤
     *      │ capacity │   3   │    |    │   2   │   y   │
     *      └──────────┴───────┘    |    └───────┴───────┘
     *                              |
     *       Stack (string_two)     |
     *      ┌──────────┬───────┐    |
     *      │   name   │ value │    |
     *      ├──────────┼───────┤    |
     *      │  pointer │ -----------|
     *      ├──────────┼───────┤
     *      │  length  │   3   │
     *      ├──────────┼───────┤
     *      │ capacity │   3   │
     *      └──────────┴───────┘
     *
     * We don't create another copy of the value in the memory because it would be too much
     * expensive in terms of memory usage. But how would the free process work? We said that when a
     * variable goes out of scope, then the compiler calls automatically the drop function, cleaning
     * the memory. Though both string_one and string_two point to the same point in memory. We can't
     * also free the memory two times, otherwise we could risk of corrupting out memory (corruption
     * may happen when trying to free a place in memory that is already free; also called "double
     * free" error). To ensure that memory gets cleaned safely, after the line
     *
     *      let string_two: String = string_one;
     *
     * then Rust doesn't consider anymore string_one as valid. If we tried to access to it, we would
     * get a compile error. What Rust does seems similar to a deep copy for all those items with
     * fixed length and similar to a shallow copy for all the items with variable length. The truth
     * is that a copy is done for elements with fixed length, while a move operation is done for
     * elements with variable length (that's what happened with the strings: string_one was moved
     * into string_two).
     */

    // In order to do a deep copy of an item, we can use the clone() method

    {
        let string_one: String = String::from("Hey there!");
        let string_two: String = string_one.clone();

        println!("String one: {:?}\nString two: {:?}", string_one, string_two);
    }

    /* Now, Rust allows to make a special type of copy for Stack-Only data. For instance, let us
     * consider the following piece of code: */

    {
        let integer_one: i32 = 6;
        let integer_two: i32 = integer_one;

        println!("Integer one: {:?}\nInteger two: {:?}", integer_one, integer_two);
    }

    /* Since it's simple to make a copy of an element in the stack (since data occupies the same
     * space), then we can do the previous lines of code, even if it seems to contradict all the
     * things said earlier. If an item has the Copy trait, then it means that it gets saved in the
     * stack. The following items allow the Copy trait:
     *  - all the integer types;
     *  - booleans;
     *  - all the floating numbers;
     *  - characters (with char);
     *  - tuples, but only if its elements allow the Copy trait (so for instance (i32, i32) allows
     *    the copy trait, but (f32, String) doesn't).
     */

    /* Functions also follow the rules of ownership: in fact, they are not that much different from
     * how assignments work. Let us consider the following example with a string and two integers:
     */

    {
        // The string gets created
        let a_string: String = String::from("Hey there! This is a string");

        // The string here can be freely used

        taking_ownership_away(a_string);    // Here the string gets passed to the function, although
        // the ownership is lost. If we try for instance to call
        // it here, we would get an error:

        // println!("{:?}", a_string);

        // This won't work with integers, because of the Copy trait that we explained earlier. Here
        // is an example:

        let first_integer: i32 = 41;
        let second_integer: i32 = 72;

        making_copy_and_sum(first_integer, second_integer);

        // Here we can use both first_integer and second_integer

        println!("{:?} - {:?} = {:?}", first_integer, second_integer,
                 (first_integer - second_integer));

        // Returning function return also the ownership of the item. So for instance, the following
        // would work:

        let another_string: String = String::from("Hey, I'm another string");

        let it_came_back: (String, i32) = returning_ownership(another_string, 43);

        println!("{:?}", it_came_back);

        /* It's a bit tedious that every time we have to transfer the ownership of some items. Rust,
         * in order to help with such problem, has a concept called borrowing and references that
         * allow us to access data even without ownership. */
    }


}

fn taking_ownership_away (a_string: String) {
    println!("{:?}", a_string);
}

fn making_copy_and_sum (an_integer: i32, to_sum: i32) {
    println!("Computing the sum between {:?} and {:?} = {:?}", an_integer, to_sum,
             (an_integer + to_sum));
}

fn returning_ownership (a_string: String, an_integer: i32) -> (String, i32) {
    // Functions can return tuples containing different data
    return (a_string, an_integer);
}