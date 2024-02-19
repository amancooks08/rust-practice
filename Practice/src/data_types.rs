pub fn run() {

    // You declare variables in Rust using the 'let' keyword.
    // You can also specify the type of the variable using a colon and the type name.
    // Rust is a statically typed language, so you need to specify the type of the variable.

    
    let unsigned: u8 = 255; // 0 to 255

    let signed: i8 = -128; // -128 to 127
    // The last bit for an integer(Ex: i8) is used to represent the sign of the number.

    let float: f32 = 3.14;

    let double: f64 = 3.14159265359;

    let boolean: bool = true;

    let character: char = 'A';

    let string: &str = "Hello, world!";

    println!("unsigned: {}, signed: {}, float: {}, double: {}, boolean: {}, character: {}, string: {}", unsigned, signed, float, double, boolean, character, string);
    // println! is a MACRO in Rust, it's like a function but with a '!' at the end. 
}