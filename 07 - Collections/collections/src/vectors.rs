/// Vectors are a data structure that allow to store an indefinite number of elements of the
/// **same types**. A vector is initialized as follows:
///
/// ```rust
/// let a_vector: Vec<i32> = Vec::new();
/// ```
///
/// Vectors are initialized with **types**, because in facts they are implemented with **generics**.
/// Generically, vectors have the form `Vec<T>`. We can also use the `vec![...]` macro, and Rust
/// will determine automatically the type of the data that we're putting inside the macro.
pub fn vectors() {
    {
        let mut v = vec![23, 44, 12];
        println!("{:?}", v);

        for item in v.iter() {
            println!("{}", item)
        }

        // Vectors can also be modified, since we can both push and pop elements out of it. For instance:

        v.push(4);
        v.push(37);
        v.push(49);

        /* If we want to read an element of the vector, we can either use the indexing way or the get() method: the
         * first one allows us to read an element by referencing it, while the second one returns an Option<T>. We'll
         * have then the following:
         */

        let an_element: &i32 = &v[2];
        println!("The element in the 3rd position is {}", an_element);

        let an_element: Option<&i32> = v.get(2);
        match an_element {
            Some(an_element) => {
                println!("The element is {}", an_element);
            }
            None => {
                println!("Got nothing out of it");
            }
        }

        // Let us consider the following program:

        /*
         * {
         *      let mut v = vec![2, 4, 7, 9, 3];
         *
         *      let first_element = &v[0];
         *
         *      v.push(48);
         *
         *      println!("The vector's first element is {}", first_element);
         * }
         *
         *
         *  Would it work? Would the Rust compiler actually compile it? It won't. This is because vectors are stored in
         *  a way such that each item is placed next to the other. If a new item must be added to the vector but in a
         *  point in memory there isn't enough space, then Rust must move all the vector to another place in memory. If
         *  that was the case, then the reference would just point to a deallocated point in the memory, resulting in an
         *  error. In order to avoid that, Rust enforces this rule.
         */

        {
            // Iterating over a vector can be done with a for loop, such as the following:
            let mut v = vec![1, 2, 3, 4, 5];

            for i in &v {
                println!("{i}");
            }

            // We can also modify elements on the go with the for loop. For instance:
            for i in &mut v {
                *i += 2;
            }

            // The previous for loop adds 2 to each element of the vector. We can see the change by printing the vector:
            println!("Modified vector: {:?}", v);

            /*  The * operator used before i in the second for loop is called dereferencing operator. It will be further
             *  explored later on
             */
        }

        /* We said that vectors can only store data of the same type: we can work around it by using enums: let the
         * following code:
         */

        {
            #[derive(Debug)]
            enum VariousVectorTypes {
                Integer(i32),
                Floating(f32),
                Boolean(bool),
                Text(String),
            }

            let v: Vec<VariousVectorTypes> = vec![VariousVectorTypes::Integer(4),
                                                  VariousVectorTypes::Floating(3.72),
                                                  VariousVectorTypes::Boolean(true),
                                                  VariousVectorTypes::Text(String::from("Hello there"))];

            // If we try to print it we'll see that it works:
            for i in &v {
                println!("{:?}", i);
            }

            /* Of course, as any other data structure in Rust, once a vector goes out of scope, it gets deleted from the
             * memory
             */
        }
    }
}