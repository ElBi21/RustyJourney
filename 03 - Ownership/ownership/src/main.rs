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
/// pushed into the stack and, when it ends, the values get popped from the stack

fn main() {
    println!("Hello, world!");
}
