#![allow(dead_code)]

// ------------------------------SECTION 1----------------------------------------------

// // This is an enum. Enums are generally same from other languages. They are helpful to create types from different languages.
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
enum Computer {
    Desktop(String),
    Laptop(String),
    Phone(String),
    Tablet(String),
}

impl Computer {
    // It's a associated function.
    fn twitter_plug(x: &str) {
        println!("Follow me on twitter! {}", x);
    }
}

fn main() {
    let computer = Computer::Laptop(String::from("Spongebob's Macbook")); // Storing the value directly in our enum.
    let twitter_account = "@is_it_ayush"; // Storing a string in the heap and getting it's ownership in twitter_account variable.
    Computer::twitter_plug(twitter_account); // Passing a reference to our string that is stored in the heap.
}
// -------------------------------------------------------------------------------------
