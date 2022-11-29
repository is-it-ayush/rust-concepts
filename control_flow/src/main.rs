/*
   I'm experimenting with rust. Trying to understand the different data types offered in the language.
   November 29, 2022
   Ayush Gupta
*/

fn main() {
    /*
       An Immutable Variable. Rust offer's immutability by default.
       - Cannot be reassigned during Runtime.
       - They're still variables. (Can be shadowed as below.)
    */
    let x = 5;

    // A mutable variable. This can be reassigned during runtime. You can store your results of functions in it.
    let mut x = 10;

    // Shadowing Over the previously declared variable.
    let x = "This is shadowing.";

    /*
        Constants: Constants are special and cannot be reassigned.
        - They can not accept a return type from a function.
        - Rust does not infer the type. So we need to declare them.
        - Standard way of defining constants is UPPERCASE: <type> = my_constant_value;
    */
    const GRAVITY: f64 = 9.8;

    /*
       Integer Overflow: Generally a unsigned 8 bit integer can only accept maximum values up to 255 [inclusive i.e. (0..=255)]
       If we assign it a value greater than 255. In the runtime, Rust performs 2's complement on it and wraps it around.
       Hence, 256 will become 0. 257 will become 1 and so on...

       Sine we have Rust Language Server running. We will get an error 'literal out of range for `u8`' which summarizes above.

    */
    let x: u8 = 255; // Change to 256 to see the error if you're running Rust Language Server.

    // Fun Fact: We can also store different integers in different notations.
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';
    let byte_string_literal = b"This is a byte string literal.";

    /*
      Common Operations In Rust:
      - Addition
      - Subtraction
      - Division
      - Multiplication
      - Remainder
    */
    let sum = 10 + 5;
    let difference = 5 - 3; // Quick Thought: What if you reversed em and explicitly defined the type to an unsigned integer. Check bottom @playground.
    let multiply = 3 * 5;
    let division = 3.5 / 5.0; // Rust ensure same type before division and won't cast itself to the greater type like java does.
    let remainder = 10 % 2;

    // Booleans
    let likes_noodles = true;
    let is_happy: bool = false; // We can still explicitly define everything in rust.

    // Characters
    let wow_its_a = 'a';
    let not_b = 'C';

    // Strings
    let a = "This is a nice string.";

    // ----------------------------------------------------------------
    // Complex data types (Not really tbh)
    // ----------------------------------------------------------------

    // Tuples
    let my_tup = (69, "I just wanna be happy.", true);
    let (number_from_tup, string_from_tup, bool_from_tup) = my_tup; // We can destructure a tuple to get it's values.
    let number_from_tup = my_tup.0; // We can also directly get the value by index. [Did you notice? This is shadowed since we declared it before too.]

    /*
      Arrays: In rust array's are of fixed length and of fixed type.
      If you want to have dynamic sized array. Use a vector. (Maybe I'll discuss later.)
    */
    let my_array = [41, 69, 420];
    let my_array_value = my_array[0]; // Accessing values directly.
    let my_new_array = [0; 8]; // This says, "Hey! Create an array with 8 values, all set to 0."

    // Playground: Here you'll find some cool edge cases and their explanation.
    /*
     This should throw an error while building since it's integer overflow.
     Observations:
     - `cargo check` does not throw the error `#[deny(arithmetic_overflow)]`
     - `rust-analyzer` does not throw the error without `  "rust-analyzer.checkOnSave.command": "clippy"` in settings.json of vscode.
    */
    let overflow: u32 = 3 - 5; // This error is intentional.
}
