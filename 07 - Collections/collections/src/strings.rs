pub fn strings() {
    /*  In Rust, there are only two types of strings: the "str" type and the "String" type. Another
     *  core difference between the two types is that the "String" type is a vector of bytes. This
     *  explains why String has a "constructor"-like function (so the new() function). */

    {
        // There are two ways to create a String: either with the .to_string() method...
        let some_text = "This is a string";

        let a_string = some_text.to_string();

        // ...or with the new() function
        let another_string = String::from("This is a string");

        println!("String 1: {:?}\nString 2: {:?}", a_string, another_string);
    }


}