#[allow(dead_code)]

// This is a struct. Generally structs are used to group similar data.
// (I'm not sure but maybe you can expand struct to structure as in giving similar data a structure.)
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // Here we initialize a (mutable) variable user1 with our User struct and also passing in the data that belongs to the struct.
    let mut user1 = User {
        email: String::from("example@test.com"), // email: String::from(""),
        username: String::from("jhondoe1"),      // username: String::from(""),
        active: true,
        sign_in_count: 1,
    };

    // Here we update the data in our user1 variable. We can do this because user1 is mutable.
    let name = user1.username;
    user1.username = String::from("isitayush");

    // Here we create another user but this time from a function that returns a new struct.
    // (We create the struct in function and take (return) the ownership [i.e. transfer the control to our variable user2]).
    let user2 = create_user(String::from("example2@test.com"), String::from("alice"));

    // This is similar to the first user creation. But instead we infer the rest of the fields from user1.
    let user3 = User {
        email: String::from("example3@test.com"),
        username: String::from("bob"),
        ..user2 // rest of the fields are copied from user2
    };

    // Tuple Struct.
    // Note: Both contain same type of data but they're both different and we cannot pass one on the other. (i.e. interchange them).
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    // ----------------------------------------------------------
    // Section 2
    // ----------------------------------------------------------

    section2();
}

// Our little helper function whose sole purpose is to create a new user and return the ownership of it back to the caller.
fn create_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 0,
    }
}

#[derive(Debug)] // What is derive debug. This is a helper for Pretty Printing (displaying) our new Struct.
struct Rectangle {
    width: u32,
    height: u32,
}

// Pseudo Explanation.
// Implementations: They're basically functions (but called methods).
// 1.) Do you have any functions that do operations on a struct? Define them as methods here.
// What we are doing is: Simply "Implementing" the functions (methods) inside the struct itself. So we don't need to explicitly call the function.
impl Rectangle {
    // A self is not necessary but if you don't pass in a self. It'll be a associated function instead of a method which is a fancy way of calling function.
    // (Associated functions are explained below).
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Here we can also pass other stuff that we require in our methods.
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Extending our previous Rectangle. We'll discuss this later "Usefulness of multiple Implementation Blocks".
// Here we are defining associated functions. They're not tied to an instance of our struct.
// How to tell the methods vs associated functions apart? Well methods get passed self and associated functions don't. - Let's Get Rusty. (https://www.youtube.com/watch?v=n3bPhdiJm9I)
impl Rectangle {
    // This is an associated function. (It doesn't take self but this function is associated to our struct, hence "associated function").
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn section2() {
    // We could do this. However, the goal is to make the code more readable and structs offer that by struct-uring our data.
    let width: u32 = 30;
    let height: u32 = 50;

    // We use a struct.
    let my_rect = Rectangle {
        width: 30,
        height: 50,
    };

    // Pretty print our "my_rect" struct.
    println!("The struct for my_rect is: {:#?}", my_rect);

    // // --------------------------Uncomment-------------------------------------------
    // // Without Tuple Implementation (Without Structs)
    // println!("The area of the rectangle is: {}", area(width, height));

    // // With Tuple Implementation (With Structs)
    // // Reason: This makes it easier to understand code. However we still have ambiguity.
    // println!(
    //     "The area of the rectangle is: {} squared pixel.",
    //     area(&my_rect)
    // );
    // // ------------------------------------------------------------------------------

    // Using implementations on structs.
    // Reason: It'll helpful if your functions perform tasks on structs themselves. Such as in case of a rectangle struct,
    // If you have to calculate area. You can simply implement the function (now method) on struct itself. It take's a reference to itself (via &self).
    // You can give those methods whatever you want which upon call will be passed in via arguments.
    println!(
        "The area of the my_rect is: {} squared pixel.",
        my_rect.area()
    );

    // Example of Using a method on a struct which is "implemented" on it that takes it another reference. Does some stuff and returns it.
    let another_rect = Rectangle {
        height: 20,
        width: 10,
    };

    // {:#?} is for pretty printing.
    println!("The struct for my_rect is: {:#?}", another_rect);

    println!(
        "The area of the another_rect is: {} squared pixel.",
        another_rect.area()
    );

    println!(
        "Can my_rect hold another_rect? {}",
        my_rect.can_hold(&another_rect)
    );

    // You can call associated functions like this.
    let rect3 = Rectangle::square(32);
}

// // --------------------------Uncomment-------------------------------------------
// // Without Tuple Implementation. (Without Structs)
// fn area(x: u32, y: u32) -> u32 {
//     x * y
// }

// // With Tuple Implementation (With Structs)
// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
// // ------------------------------------------------------------------------------
