/// Structs are a way to aggregate multiple data types to create a custom data type. It's similar to
/// the set of attributes of an object in any OOP language (like Java). They are similar to tuples
/// in some sense, but they don't really require any order when being made.
pub(crate) fn structs_general() {
    // A struct can be defined this way:

    struct User {
        name: String,
        age: usize,
        email: String,
        online: bool
    }

    /* The struct name should be with the first letter capital. A struct, when declared, has fields,
     * which are the pairing of names and types of data that we are using. In order to implement the
     * struct, we have to create an instance of it. We can do it this way:
     */

    {
        let mut its_me: User = User {
            name: String::from("Leonardo"),
            age: 19,
            email: String::from("example@example.org"),
            online: false
        };

        /* Once an instance has been created, if we declared it as mutable, then we can edit its
         * contents. An instance of a struct can't have some mutable fields and the others immutable
         * or vice versa: they all have to be either mutable or immutable. For instance, here we set
         * the `online` boolean to true:
         */

        its_me.online = true;

        // Each instance is independent from the others. We can test it by looking at the following:

        let another_user = User {
            name: String::from("Joe"),
            age: 20,
            email: String::from("joe.dragon@matrix.com"),
            online: false,
        };

        // If we try to print the two online statuses, we'll have different results:

        println!("Is {} online? {:?} | Is {} online? {:?}",
                 its_me.name, its_me.online, another_user.name, another_user.online);

    }

    // We can also create a "constructor" for the struct with a function

    fn new_user (name: String, age: usize, email: String) -> User {
        User {
            // We can simplify the expression in this way:
            // name: name,
            name,
            // age: age,
            age,
            // email: email,
            email,
            online: false,
        }
    }

    {
        let ken: User = new_user(String::from("Ken"), 35,
                                 String::from("kenough.ken@mattel.org"));

        /* We can also use the attributes of an already created instance, either by listing each
         * single attribute or by using the syntax `..prev_instance`. Such syntax will fill the
         * remaining fields with the values of the previous instance passed
         */

        let another_ken: User = User {
            email: String::from("ken2.themarvelous@mattel.org"),
            online: ken.online,
            ..ken
        };

        /* Using values of other structs moves the values inside the memory. What is happening is
         * that we are basically reassigning the values from one struct to the other. We would have
         * an error then if we tried to print the name of the first instance:
         */

        // println!("{}", ken.name);
        println!("{}", another_ken.name);

        /* We can also make tuple structs: such structs are similar to tuples, with the difference
         * that the fields have no labelled name. This is useful while making structs where the name
         * is not necessary / is redundant.
         */

        struct Color (i32, i32, i32);

        let bordeaux: Color = Color(74, 9, 29);

        /* In order to access to a tuple struct instance attribute, we just do the same as we would
         * do with a tuple:
         */

        println!("{:?}", bordeaux.0);

        /* We can also have what we call Unit-Like Structs, which are structs with no fields. They
         * are useful when we want to implement a trait on some type that we make, but without the
         * employment of actual data. We'll see further how to do it and why it's useful. For the
         * moment, it's enough to say that they exist. They are written like the following:
         */

        struct AlwaysEqual;

        let a_unit: AlwaysEqual = AlwaysEqual;

        /* For example, a Unit-Like struct may be helpful if we want to do a struct (like
         * AlwaysEqual) that is always equal to the type f any other instance. Again, we'll see
         * later on how to do it, but it's enough to know that this is possible.
         */
    }
}