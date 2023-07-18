/// When passing variables into functions, we usually lose the ownership on them, making them
/// unavailable after the ownership gets lost, unless they get saved in the stack. We can sort of
/// avoid this losing mechanism by using references. A **reference** is like a pointer that points
/// to a specific data in the memory; such data is owned though by another variable.
pub(crate) fn references () {
    // An example of reference is the following:

    {
        let string_one: String = String::from("A new string!");

        let string_len: usize = get_length(&string_one);

        println!("The string {:?} is {:?} characters long", string_one, string_len);
    }

    /* References are made with the & keyword. Once a reference goes out of scope, the value of the
     * reference it's not dropped. When we create a function that will take a reference, we have to
     * specify it. In the case of get_length, we wrote that string is of type &String, so a
     * reference of a string. The process of creating a reference is called borrowing: since we
     * never owned such variable, we don't need to care about returning the ownership. Reference
     * though, by default, can't be mutated. Once a reference of a variable gets created, then such
     * reference can't be edited. If we tried to do it, we would get an error. We can though edit a
     * reference just by doing &mut Type: this way, with &mut we are telling Rust that the reference
     * can be edited, and with Type we are specifying its type. An example is given by the function
     * edit_string():
     */

    {
        let mut a_string: String = String::from("Hello world");

        edit_string(&mut a_string);

        println!("{:?}", a_string);
    }

    /* There is an important fact about references: there can exist only one reference to a specific
     * variable at a time, and no more than one. If we tried to make two references, we would get an
     * error. This behaviour occurs because Rust tries to avoid Data Races. Data Races are similar
     * to race conditions, but with data. They normally occur in three cases:
     *  - If two or more pointers try to access to the same data at the same time;
     *  - If two or more pointers exist and at least one is trying to write some data;
     *  - If there is no mechanism that manages the way data is accessed.
     *
     * Mutable references cannot exist also if we already have immutable references, and that's for
     * the reasons explained earlier. References last from the first reference up until the last
     * reference pointing to the same data.
     *
     * In some languages with pointers, it's easy to create dangling pointers: pointers that point
     * to some places in memory that are occupied by some other data. For instance, suppose that in
     * the memory, at an address X, we had a string my_string, and that such string went out of
     * scope at a certain point of the code. If the pointer to such memory address still persists,
     * then if we try to use it to call the string, we surely won't get the original string, but
     * rather something else. This is a dangling pointer. An example is given by the function
     * dangle(). The function is commented because otherwise it would return an error at compile
     * time. Recapping, the two reference rules are the following:
     *  - At any given time, for one piece of data, there can exist either one immutable reference
     *    or one mutable reference of it;
     *  - References must always be valid, otherwise a dangling pointer would be made.
     */
}

/// Simple function illustrating how references work
fn get_length(string: &String) -> usize {
    return string.len();
}

/// Simple function that lets us observe the behaviour of `&mut`
fn edit_string(string: &mut String) {
    string.push_str("[NEW CONTENT]");
}

// Function that explains the concept of **dangling pointers**
/* fn dangling_pointers() -> &String {
    let string: String = String::from("I'm a weird string");

    return &string;
} */