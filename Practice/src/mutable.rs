pub fn run() {
    
    let immutable = 10;
    println!("immutable: {}", immutable);

    // immutable = 20;
    // Try uncommenting the above line and see what happens.

    // You can't reassign a value to the variable, and probably got a compilation-error right?

    // This is because there's this cool thing in Rust that not every declared variable is mutable by default.
    // You need to specify the 'mut' keyword to make a variable mutable.
    // This is a good practice as it makes the code more readable and less error-prone.

    // Now let's see how to make a variable mutable.
    let mut mutable = 20;
    println!("mutable: {:?}", mutable);

    mutable = 30;
    println!("mutable: {}", mutable);
    // Now you can reassign the value to the variable. Cool, right?
}