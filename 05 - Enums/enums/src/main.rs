use std::f32::consts::E;
use std::net::Ipv6Addr;

/// Rust allows the use of enumerations (or enums), which are a way to describe a type by
/// enumerating all the possible variants of such type.
fn main() {
    /* Structs allow us to make our custom data types, but enumerations allow us to make a kind of
     * "super-type" that contains multiple types. For instance, IP addresses: there are two types of
     * IP address: V4 and V6. If we had to make a collection of IP addresses, we would make an
     * enumeration with all the possible IP types. Indeed, the enumeration would be similar to a
     * super-type, because it brings together multiple types. Let's make an example:
     */

    {
        enum IPAddress {
            // Types should follow the "Camel Case" rule
            V4,
            V6,
        }

        /* IPAddress is, under all the aspects, a type. We can also create an instance of IPAddress
         * and specify which of the two variant we want to select:
         */

        {
            let addr_v4: IPAddress = IPAddress::V4;
            let addr_v6: IPAddress = IPAddress::V6;

            /* Notice how the type is only the enum and NOT the detailed down to the variant.
             *
             * If we make a function that takes as a parameter an IPAddress, then it doesn't matter
             * whether the address is an IPv4 or IPv6.
             */

            // Generic parameter
            fn check_address(ip_address: &IPAddress) {
                // Some code here...
            }

            // Specific parameter
            fn check_v6_address(ip_address: &IPAddress::V6) {
                // Some code here...
            }

            check_address(&addr_v4);
            check_address(&addr_v6);
        }

        /* How can we attach some data to the enumeration? A way could be for instance to create a
         * struct, and make one of its fields define the type of the address. So something like the
         * following may happen:
         */

        {
            struct IPAddr {
                kind: IPAddress,
                address: String,
            }
            ;

            let an_address: IPAddr = IPAddr {
                kind: IPAddress::V4,
                address: String::from("127.0.0.4"),
            };

            let loopback_address: IPAddr = IPAddr {
                kind: IPAddress::V6,
                address: String::from("::1"),
            };
        }
    }

    /* This concept will work, but we can do something even more concise: we can add directly some
     * data inside each enum variant. A newer version is here shown:
     */

    {
        {
            enum IPAddress {
                V4(String),
                V6(String),
            }

            //A way to write data on an instance is like the following:

            let an_address: IPAddress = IPAddress::V4(String::from("127.0.0.4"));

            let loopback_address: IPAddress = IPAddress::V6(String::from("::1"));
        }

        /* Notice how the fields of the enum are treated as functions: this is because they became a
         * kind of "constructors", and they allow to prepare the data that will go inside each
         * field.
         *
         * Enums allow us to define for each field multiple and different types of data. For
         * instance, with one same struct we would never be able to express both an IPv4 and IPv6
         * address, we would need two different struct. Enums allow us to do it though. For
         * instance, we could to the following:
         */

        {
            enum IPAddress {
                V4(u8, u8, u8, u8),
                V6(String),
            }

            // This way we would have the following:

            let an_address: IPAddress = IPAddress::V4(127, 0, 0, 4);
            let loopback_address: IPAddress = IPAddress::V6(String::from("::1"));
        }

        // The implementation of IP addresses in the standard Rust library is like the following:

        {
            struct IPv4Addr {
                address: (u8, u8, u8, u8),
            }

            struct IPv6Addr {
                address: String,
            }

            enum IPAddress {
                V4(IPv4Addr),
                V6(Ipv6Addr),
            }
        }
    }

    // Let's analyze the following enumeration:

    #[derive(Debug)]
    enum Message {
        Quit,
        Move {
            x: i32,
            y: i32,
        },
        Write(String),
        ChangeColor(i32, i32, i32, i32),
    }

    /* Such enumerations is quite interesting, let's see all the fields one by one:
     *  - Quit: has no data associated with it;
     *  - Move: has some name fields, similarly to a struct;
     *  - Write: its data type is String;
     *  - ChangeColor: the associated data type consists of a tuple of 3 i32 numbers.
     *
     * Such enumeration could've been made with several different structs (note the scope below),
     * but that way we would have had troubles to define a function that would work with all the
     * types.
     */

    {
        struct QuitMessage;     // Unit struct
        struct MoveMessage {
            x: i32,
            y: i32,
        };      // Standard struct
        struct WriteMessage(String);    // Tuple struct
        struct ChangeColorMessage(i32, i32, i32);   // Tuple struct
    }

    /* We can use `impl` not only with structs, but also with enums. This allows to extend even
     * more the possibilities of the enums.
     */

    impl Message {
        fn send_to_console (&self) {
            println!("{:?}", self);
        }
    }

    {
        let my_message: Message = Message::Write(String::from("Houston, we got a problem..."));
        my_message.send_to_console();
    }

    /* In a lot of programming languages, there is something similar to a `null` value: in Python is
     * `None`, in Java is `null`, and so on...
     *
     * In Rust there is no such thing as a `null` value, but there is a way to implement the
     * possibility of having an absent value. We can do that with Option<T>. Option<T> is
     * automatically included in the prelude, and it's an enumeration that looks like the following:
     *
     * enum Option<T> {
     *      None,
     *      Some(T),
     * }
     *
     * <T> is a generic type parameter, similar to Java's. Here follows an example of Option:
     */

    {
        // Here we use Some to insert some data
        let a_number: Option<i32> = Some(41);
        let a_string: Option<String> = Some(String::from("Hello there"));

        // In order to print the data in Some, we must use the Debug print (thus `{:?}`)
        println!("{:?} | {:?}", a_number, a_string);

        // Here is an example of a None value
        let an_absent_value: Option<i32> = None;
    }

    // Why should we use Option<T> and not directly a null value? Consider the following case:

    {
        let a_number: Option<i32> = Some(5);
        let another_number: i32 = 4;

        println!("Calculating the sum of {:?} and {:?}... (but can I actually do that?)",
                    a_number, another_number);
        // let the_sum = a_number + another_number;
        /* This line won't execute because we are trying to add two different elements of different
         * types. Similarly to Java, Option<T> is not always the same of T. The reason why the None
         * value is locked behind an enum is to ensure safety. If we want to take the value from the
         * Some() case, we can use various methods of the Option enumeration. They are listed here:
         *
         *  -> https://doc.rust-lang.org/std/option/enum.Option.html
         *
         * The other reason why None can be used only with Option is because that way we have a
         * specific way to handle null values, ensuring that they will always be treated carefully,
         * thus increasing the safety of Rust's code. A way to treat such cases is via the use of
         * the `match` keyword.
         *
         * The `match` keyword allows for the use of this powerful control flow construct, which
         * allows to handle all the possible cases of an enumeration with ease. It may resemble a
         * Finite State Machine (FSM): depending on the input X, the FSM will jump into different
         * states, executing different functions depending on the case. For instance, let the
         * following enumeration and its relative match statement:
         */

        {
            enum Euros {
                Cent,
                EuroCoin,
                EuroBanknote,
            }


            fn print_values (euros: Euros) {
                // Match works with an instance of the enum
                match euros {
                    Euros::Cent => {
                        println!("0.01€ | 0.02€ | 0.05€ | 0.10€ | 0.20€ | 0.50€");
                    },
                    Euros::EuroCoin => {
                        println!("1.00€ | 2.00€");
                    },
                    Euros::EuroBanknote => {
                        println!("5.00€ | 10.00€ | 20.00€ | 50.00€ | 100.00€ | 200.00€ | 500.00€ (D)")
                    }
                }
            }

            /* Given different instances of the Euros enum, we can call the function and look at
             * what happens:
             */

            let a_cent: Euros = Euros::Cent;
            print_values(a_cent);

            let an_euro_coin: Euros = Euros::EuroCoin;
            print_values(an_euro_coin);

            let an_euro_banknote: Euros = Euros::EuroBanknote;
            print_values(an_euro_banknote);
        }

        /* Match can also be useful to extract the value of an enum variant. Moreover, match can
         * bind to the values of a particular variant. For instance, let's write again the Euros
         * enum with a little modification: the cents and the coins have different designs depending
         * on the state that minted it. We'll thus have two enums: one for the coins, and one for
         * the states:
         */

        {
            #[derive(Debug)]
            enum EUStates {
                Italy,
                France,
                Belgium,
                Germany,
                Spain,
                Portugal,
                Netherlands,
                Sweden,
                // and so on...
            }

            enum Euros {
                CentCoin(EUStates),
                EuroCoin(EUStates),
                EuroBanknote,
            }

            fn return_values(coin: Euros) -> String {
                match coin {
                    Euros::CentCoin(state) => {
                        println!("It comes from {:?}", state);
                        String::from("0.01€ | 0.02€ | 0.05€ | 0.10€ | 0.20€ | 0.50€")
                    }
                    Euros::EuroCoin(state) => {
                        println!("It comes from {:?}", state);
                        String::from("1.00€ | 2.00€")
                    }
                    Euros::EuroBanknote => {
                        String::from("5.00€ | 10.00€ | 20.00€ | 50.00€ | 100.00€ | 200.00€ | 500.00€ (D)")
                    }
                }
            }

            let a_coin: Euros = Euros::EuroCoin(EUStates::Belgium);
            let another_coin: Euros = Euros::CentCoin(EUStates::Italy);
            let a_banknote: Euros = Euros::EuroBanknote;

            println!("{}", return_values(a_coin));
            println!("{}", return_values(another_coin));
            println!("{}", return_values(a_banknote));
        }

        /* The match statement can also be used with the Option<T> enum. Since the match statement
         * is exhaustive (meaning that wants all the possible variants have to be treated), we have
         * to treat both the Some and the None variants.
         */

        {
            fn add_one(a_number: Option<i32>) -> Option<i32> {
                match a_number {
                    Some(item) => Some(item + 1),
                    None => None,
                }
            }

            let number: Option<i32> = Some(5);
            let empty: Option<i32> = None;

            let update_number: Option<i32> = add_one(number);
            let update_null: Option<i32> = add_one(empty);

            println!("{:?} | {:?}", update_number, update_null);
        }

        /* In case an enum has too much possibilities, then we can handle them differently: we can
         * specify a case that will be valid for every other case: say for instance that we have a
         * function that rolls a dice: if we roll 1 we do something, if we roll 4 we do something
         * else and, if we roll any other number, we do something different. We can do the
         * following:
         */

        {
            let my_roll: i32 = 3;

            fn action_on_roll(roll: i32) {
                match roll {
                    1 => println!("Lucky one! Roll again"),
                    4 => println!("Unlucky! Go back of 4 spaces"),
                    any_other_number => println!("Way to go! Move your player of {} spaces",
                                                        any_other_number),
                }
            }

            action_on_roll(my_roll);
        }

        /* In this case, we needed the value of the roll, since `my_roll` won't be available
         * anymore; if we didn't need it we would've used the `_`, which is a catch-all symbol. This
         * symbol will automatically tell Rust that we won't use the value that it holds, thus it
         * can disregard it. An example follows:
         */

        {
            let my_roll: i32 = 5;

            fn action_on_roll(roll: i32) {
                match roll {
                    1 => println!("Lucky one! Roll again"),
                    4 => println!("Unlucky! Go back of 4 spaces"),
                    _ => println!("Way to go! You can move of one space"),
                }
            }

            action_on_roll(my_roll);
        }

        /* In some cases the match statement can be a bit wordy. The `if let` statement allows us to
         * better handle cases where we more specifically care about the value. For instance,
         * consider this match statement:
         */

        {
            let configurable_max: Option<u8> = Some(3u8);
            match configurable_max {
                Some(max) => println!("The max is {}", max),
                _ => (),
            }

            /* With the match statement we have to handle all the cases: this means that we also
             * have to handle the _ case. This just adds unnecessary lines of code to the project.
             * This can be also done with the `if let` statement:
             */

            {
                let configurable_max: Option<u8> = Some(3u8);

                if let Some(given_max) = configurable_max {
                    println!("The max value exists, and it's {}", given_max);
                } else {    // We can also add an else to treat all the other cases
                    println!("The max value doesn't exist");
                }
            }
        }
    }
}