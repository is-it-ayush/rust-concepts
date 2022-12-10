#![allow(dead_code)]

// ------------------------------SECTION 1----------------------------------------------

// // This is an enum. Enums are generally same from other languages. They are helpful to create types of a specific object that could be have multiple forms.
// enum IpAddrType {
//     V4,
//     V6,
// }

// // This is a struct. A struct is a way to "struct"ure your data. (that is organizing related data together).
// struct IpAddress {
//     version: IpAddrType,
//     address: String,
// }

// fn main() {
//     let ipv4 = IpAddrType::V4; // We can
//     let ipv6 = IpAddrType::V6;

//     // Creating a struct with one of the types being our defined enum.
//     let localhost = IpAddress {
//         version: IpAddrType::V4,
//         address: String::from("127.0.0.1"),
//     };
// }

// // This means we can pass our enums too.
// fn route(ip_kind: IpAddrType) {
//     // do something...
// }
// -------------------------------------------------------------------------------------

// ------------------------------SECTION 2----------------------------------------------
// // We can also directly store data inside our enums.
// // To store data inside enums we use brackets and then define the type of data we want to store.
// enum IpAddrType {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// fn main() {
//     // The same thing happens here. We simply pass in our data and our enum will hold onto it.
//     let localhost = IpAddrType::V4(127, 0, 0, 1);
//     let a_test_ipv6 = IpAddrType::V6(String::from("fd8f:be16:58b6:51c5::1"));
// }

// -------------------------------------------------------------------------------------

// ------------------------------A Bit Of Explanation----------------------------------------------
// Broadly speaking, An enum is basically used to group multiple structs. We can also define methods and associated functions
// onto an enum.
// -------------------------------------------------------------------------------------

// ------------------------------Section 3----------------------------------------------
// enum Computer {
//     Desktop(String),
//     Laptop(String),
//     Phone(String),
//     Tablet(String),
// }

// impl Computer {
//     // It's a associated function.
//     fn twitter_plug(x: &str) {
//         println!("Follow me on twitter! {}", x);
//     }
// }

// fn main() {
//     let computer = Computer::Laptop(String::from("Spongebob's Macbook")); // Storing the value directly in our enum.
//     let twitter_account = "@is_it_ayush"; // Storing a string in the heap and getting it's ownership in twitter_account variable.
//     Computer::twitter_plug(twitter_account); // Passing a reference to our string that is stored in the heap.
// }
// -------------------------------------------------------------------------------------

// ------------------------------Section 4----------------------------------------------

fn main() {
    // // There are no null value in Rust. Instead we have None inside Option Enum. If we have a value that could either be null or some type.
    // // We'll wrap that value inside option enum. This forces us to handle to none case.
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    // let x = Some(10);
    // let y = Some("Rust will infer the type automatically.");

    // let absent_value: Option<i32> = None;

    // // Adding Option to a normal value.
    // let x = 5;
    // let y = Some(5);

    // // Here we provide a deafault value just in case our Option value is Absent.
    // let sum = x + y.unwrap_or(0);

    // ---- An Example Of Match Expression ----
    #[derive(Debug)]
    enum State {
        Alabama,
        Arkansas,
        Arizona,
        California,
        Ohio,
        //...
    }

    enum Coin {
        Penny(State),
        Nickle(State),
        Dime(State),
        Quarter(State),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny(state) => {
                println!("Minnted In: {:?}", state);
                1
            }
            Coin::Nickle(state) => {
                println!("Minnted In: {:?}", state);
                5
            }
            Coin::Dime(state) => {
                println!("Minnted In: {:?}", state);
                10
            }
            Coin::Quarter(state) => {
                println!("Minnted In: {:?}", state);
                25
            }
        }
    }

    value_in_cents(Coin::Nickle(State::California));

    // ---- Another Example ----
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_value = Some(3);

    // We could write it with match.
    match some_value {
        Some(5) => print!("some value is equal to another value."),
        _ => (),
    }

    // However, Since we are only handling one (or a few) case and if we don't care about all other cases.
    // We can rewrite it via if-let syntax. 
    if let Some(2) = some_value {
        println!("They're equal.")
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
