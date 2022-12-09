// So What is Ownership?
/*
   Ownership Rules:
   - Each value in Rust has a variable that's called it's owner.
   - There can only be one owner at a time.
   - When the owner goes out of scope, the value will be dropped. (Automatic Memory Management, We don't need to worry about deallocation)
   #Reference: https://youtu.be/VFIOSWy93H0 (Let's Get Rusty)
*/

fn main() {
    {
        // s is not valid here. (scope start)

        // s is defined here. s is allocated in the heap and a pointer is stored in the stackframe of main() function.
        let s = String::from("hello");

        // s is valid here
    } // s is not valid here. (scope end) (deallocation of everything inside {} )
}

fn copy() {
    // Stack Allocation
    let x = 10;
    let y = x; // This will copy the value from x and store it in y. Rust has a copy trait stored on the stack.

    // Heap Allocation. (Interesting)
    let a = String::from("hello");
    let b = a; // A Move Happens.

    /*
       You might argue that "oh, it's just another copy" or "oh, is it shallow copy? (both a and b referencing to the String hello)"
       But Neither happens. When you do this for heap. Rust invalidates a. Now b has entire control of the string in heap and a is no longer valid.
       This is called a "move" i.e. moving the declaration from a to b in heap.
    */

    // What if we actually wanted to copy from heap?
    // Well, do this.
    let c = b.clone(); // This will copy instead of moving the declaration. Cloning is expensive.
}

// ------------------------------------------------------------------------------------------------------------------------------------------

fn initial_ownership() {
    let s = String::from("Hello");
    take_ownership(s);
    // println!("{}", s); // Uncomment this line.
    // Error's Out `borrow of moved value: `s`.
    // Why? Generally when you pass the value to a function. Rust performs a move instead of a copy to that argument.
    // This is only valid for the types in the heap. The stack one's are still copied and can be used.
}

fn take_ownership(x: String) {
    println!("{}", x); // Now x has control over the string in the heap.
}

fn give_ownership() -> String {
    String::from("Hi!") // Returning a string. This generally transfer's the control back to the caller variable. This is the vice_versa of take_ownership.
}

fn take_and_give_back_ownership(x: String) -> String {
    x // Return the string after taking it. We have a lot of control over memory.
      // This is still tedious.
}

// Here we are referencing to the passed in argument. Say We pass in,
// let y = String::from("Hello!");
// referencing_and_not_moving(y); // Here x will reference y and y will point to the actual data. We are not modifying y and x will be dropped after function does executing.
// Note: references are immutable by default. We CANNOT modify them.
fn referencing_and_not_moving(x: &String) -> usize {
    x.len()
}

// What if we do want to mutate? Then we have to specify mut and pass a mutable reference.
fn mutate_refrencing_without_taking_ownership(x: &mut String) {
    x.push_str("world");
    // This will work but the passed in argument must be mutable.
}

// However: You can only have one mutable reference at one time. What does it mean?
// Let's call mutate_refrencing_without_taking_ownership
fn restriction_one() {
    let mut a = String::from("Hello, ");
    let r1 = &mut a;

    // This will not work. As You can only have one reference to a particular variable at one time.
    // This is by design to prevent data races.
    // A data race occurs when two pointers, point to the same data and one of them writes to the data & there is no mechanism to sync them.
    // let r2 = &mut a; // Uncomment this line

    mutate_refrencing_without_taking_ownership(r1);
}

// It's okay to have multiple immutable references, but not multiple mutable references because data races. (Explained above).
fn restriction_two() {
    let mut a = String::from("Hello");

    let b = &a;
    let c = &a;

    // let d = &mut a; // We also can't have both immutable and mutable references together. Uncomment to see the error.

    println!("{} , {}", b, c);
}

// fn restriction_three() {
//     let a = restriction_three_helper();
// }

// // Rust prevents dangling data. It'll error out.
// // Dangling pointers, point to nothing i.e. they point to places which are deallocated by rust at scope end.
// fn restriction_three_helper() -> &String {
//     let s = String::from("This will be dropped and the reference will point to nothing.")
//     &s
// }

/*
   General Rules Of References:
   - At any given time, we can have one mutable reference or so many (as much as we want) immutable references.
   - References must always be valid. (Must not be dangling)
*/

// ----------------------------------------------------------------------------------------------------------------------
