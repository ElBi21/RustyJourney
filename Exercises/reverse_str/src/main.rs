fn main() {
    // let to_reverse: String = String::from("La patata è carina");
    let to_reverse: String = String::from("uüu");

    let mut result: String = String::new();

    for character in to_reverse.chars() {
        result.insert(0, character);
    }

    println!("{:?}", result);
}