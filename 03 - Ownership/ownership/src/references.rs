use std::io::Read;

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

    /* The Slice type allows us to refer to a contiguous sequence of elements in a collection,
     * rather than the whole collection. For instance, we want to make a function that returns the
     * first word in a string with spaces (so the word before the first space), but if there are no
     * spaces, then we want to return the whole string. Let's see how to do that:*/

    fn first_word_before_space(string: &String) -> usize {
        /* Since we can't really iterate on a string like in Python, we have to find another way:
         * for instance, we could try to return the index of the last space. We can do so by
         * transforming our string into an array of bytes, and then we could iterate on the array
         * and check for the first space
         * */

        let as_bytes = string.as_bytes();

        // Here we convert the bytes array into an iterator, and then we enumerate the items
        for (i, &item) in as_bytes.iter().enumerate() {
            // If the item is a space, we then proceed to return its index
            if item == b' ' {
                return i;
            }
        }

        // Otherwise we can just return the length of the string
        return string.len()
    }

    /* It's important to say that each element of .iter().enumerate() is a tuple with the following
     * form:
     *
     * (index, &item)
     *
     * The index is a variable that can be owned, while the item is a reference.
     *
     * Let us now try to call the function with any string:
     */

    {
        let mut a_string: String = String::from("Here I am, standing in front of you");

        let word_index: usize = first_word_before_space(&a_string);

        a_string.clear();

        println!("The index of the first space is {:?}", word_index);

        /* The problem with the function is that the index is "valid" for the string that we set
         * before, but if the string changes, then the index is not coherent anymore.*/
    }

    /* There is a way to select a part of a string, just like in Python. The way to do it is via
     * accessing to the reference to the string and then specify the index of the parts of the
     * string that we want. An example follows:
     */

    {
        let a_string: String = String::from("Hello, my guy");

        let part_one: &str = &a_string[0..5];
        let part_two: &str = &a_string[10..13];

        println!("{:?} and {:?}", part_one, part_two)
    }

    /* The slicing indexes follow the rule
     *
     *     [beginning_index..(ending_index + 1)]
     *
     * We can omit the indexes, meaning that they will be equal to 0.
     */

    {
        let a_string: String = String::from("Hey hey hey! Who do we have here?");

        let example_one: &str = &a_string[..3];
        let example_two: &str = &a_string[4..a_string.len()];
        let example_three: &str = &a_string[13..];

        println!("{:?}\n{:?}\n{:?}", example_one, example_two, example_three);
    }


}