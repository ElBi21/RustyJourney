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
     *
     * We can try to use structs to have a more concise program.
     */

    {
        let my_rectangle: Rectangle = Rectangle {
            width: 36,
            height: 40,
        };

        println!("The area with the struct is {:?}", get_area_rectangle(&my_rectangle));


        /* While dealing with structs, it would be a nice thing to have a standardized way of
         * printing an instance of any struct. This way our debugging purposes would be way more
         * accessible. By default, Rust will return an error if we try to directly print an
         * instance. The `println!` macro acts depending on what's inside the curly brackets. For
         * instance, empty curly brackets mean that the element that will be printed will be shown
         * with the `Display` trait. If we put `{:?}`, then we want to use the `Debug` trait. If we
         * use it with an instance though, the compiler will suggest us something useful:
         */

        // println!("{:?}", my_rectangle);

        /* The compiler output the following lines:
         *
         *  = help: the trait `Debug` is not implemented for `Rectangle`
         *  = note: add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`
         *
         * If we add `#[derive(Debug)]` on the line before declaring the struct, we will implement a
         * way to print the rectangle. If we now make an instance of DebugRectangle, we'll be able
         * to print it:
         */

        let a_rectangle: DebugRectangle = DebugRectangle {
            width: 36,
            height: 40,
        };

        println!("{:?}", a_rectangle);

        // We can use `{:#?}` for a more expanded debug print

        println!("{:#?}", a_rectangle);

        /* The `dbg!` macro can be used as well. In contrast to `print!` though, `dbg!` takes the
         * ownership of an expression (and then returns it), while `print!` takes a reference. Plus,
         * `dbg!` prints to the `stderr` (Standard Error Console), and not to the `stdout` (Standard
         * Output Console).
         */

        let factor: u32 = 6;
        let debug_rectangle: DebugRectangle = DebugRectangle {
            width: dbg!(30 + factor),
            height: 40
        };

        dbg!(&debug_rectangle);
    }

    /* In Rust, similarly to OOP languages, we can also do methods. Methods are specific functions
     * made only for some elements. This is important because it helps keeping the code clean and
     * keeps some functions for only some types. We can implement a method with the `impl` keyword.
     * An example is shown after the Rectangle struct.
     *
     * Now we can use the area method to directly get the area of any rectangle, keeping the code
     * clean. An example follows:
     */

    {
        let a_rectangle: Rectangle = Rectangle {
            width: 36,
            height: 40,
        };

        println!("The area with the method is {:?}", a_rectangle.area());

        // A method can have the same name of a field. For instance:

        println!("A statement says that the width of our rectangle is more than 0. That is {:?}",
                    a_rectangle.width());

        /* Similarly to Java, encapsulation is a thing in Rust. The possibility to make methods have
         * the same name of some fields is to make some getters methods (so methods that return the
         * value of a field), and that is useful for instance if we make a field private.
         *
         * Rust has a feature called "automatic referencing and dereferencing", which autocompletes
         * during compile time the call to a method, let it be a reading method (with `&self`), a
         * mutating one (with `&mut self`) or consuming (with `self`).
         *
         * When creating methods that take more than one parameter (one more than `&self`) we can
         * just put them inside the `()`, as we would do for any other function. If we include also
         * `&self`, then when calling the method we just have to fill the parameters that are not
         * `&self`. An example follows:
         */

        let a_second_rectangle: Rectangle = Rectangle {
            width: 33,
            height: 21,
        };

        let a_third_rectangle: Rectangle = Rectangle {
            width: 13,
            height: 41,
        };

        println!("Can a_rectangle hold a_second_rectangle? {}",
                 a_rectangle.can_fit(&a_second_rectangle));

        println!("Can a_rectangle hold a_third_rectangle? {}",
                 a_rectangle.can_fit(&a_third_rectangle));
    }

    /* We call associated functions all those functions that are associated to a specific type with
     * the `impl` keyword. We can also make associated functions that don't need a `self` parameter.
     * An example of such function is the `String::from()` function. We can make an example of such
     * type of function by doing a function that creates a square using the `Rectangle` struct.
     */

    {
        let my_square: Rectangle = Rectangle::square(41);

        println!("My square has the following side size: {:?}\nThe area is {:?}",
                 my_square.width, my_square.area());
    }

    // Each struct can have multiple `impl` blocks
}

fn get_area(width: i32, height: i32) -> i32 {
    width * height
}

fn get_area_tuples(dimensions: (i32, i32)) -> i32 {
    dimensions.0 * dimensions.1
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    /* `self` has the same meaning of `self` in Python or `this` in Java: it refers to the instance
     * that is calling the method. `&self` stands for `self: &Self`.
     */
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_fit(&self, to_fit: &Rectangle) -> bool {
        self.width > to_fit.width && self.height > to_fit.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
        }
    }
}

fn get_area_rectangle(rectangle: &Rectangle) -> u32 { rectangle.width * rectangle.height }

#[derive(Debug)]
struct DebugRectangle {
    width: u32,
    height: u32,
}