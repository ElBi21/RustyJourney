pub(crate) fn structs_ownership_and_examples() {
    /* The concept of ownership also applies to structs. Before, in `structs_general`, we found out
     * that the concept of borrowing applies to structs fields. We also mentioned that it's possible
     * to store references, but that we won't see it now. This is because we would need a lifetime,
     * a concept of Rust that will be explained later. Usually, lifetimes allow for references to
     * exist as long as the struct is valid. Without the use of lifetimes, we would get a compile
     * error.
     *
     * In order to understand when to use structs or not, we can try to build a simple program that,
     * given the width and height of a rectangle, it will compute the area. This will be done first
     * with a simple function, then with a struct
     */

    {
        let width: i32 = 36;
        let height: i32 = 40;

        println!("The area is {:?}", get_area(width, height));
    }

    /* This code is pretty straight forward: given two parameters, it makes the multiplication
     * between them, and since the multiplication doesn't really ask for a particular order, it's
     * not important which of the two is the width or the height parameter. Problems may start to
     * arise when we want the data to be related, although we can't really keep any relation if we
     * use the two parameters like that. For instance, it may happen that we want to do a more
     * general function that has different paths depending on the figure. If we pass only width and
     * height, then there is no way that would allow us to determine of which figure we are talking
     * about. A way to try to keep some consistency would be for instance to use tuples:
     */

    {
        println!("The area with the tuples is {:?}", get_area_tuples((36, 40)));
    }

    /* Still, tuples have the problem that don't label their items: for us it's clear because we
     * know the context of the function, but it couldn't be obvious for other programmers.
     */

}

fn get_area(width: i32, height: i32) -> i32 {
    width * height
}

fn get_area_tuples(dimensions: (i32, i32)) -> i32 {
    dimensions.0 * dimensions.1
}