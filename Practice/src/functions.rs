pub fn run() {
    let num = 10;
    println!("Is {} even? {}", num, is_even(num));
}

// In Rust, you define functions using the 'fn' keyword, 
// followed by the name of the function, then the parameters of the function,
// then the return type of the function, and then the body of the function.
 
// There are also access specifiers in Rust,
// which are used to specify the visibility of the function.
// The access specifiers are 'pub' and 'priv'.
// 'pub' is used to specify that the function is public,
// and 'priv' is used to specify that the function is private.
// If you don't specify any access specifier, then the function is private by default.

// Let's see how to define a function in Rust.

pub fn is_even(num: i32) -> bool {
    // This is a public function to check if a number is even or not.
    // The function takes an integer as a parameter and returns a boolean value.
    // The function body contains the logic to check if the number is even or not.
    // The function returns true if the number is even, and false if the number is odd.
    let digit = num % 2;

    digit == 0 
    // This is the return value of the function.
    // Wondering why there is no return keyword?
    // In Rust, the last expression of the function is the return value of the function.
    // And to make it count as a return statement, you don't need semi-colons.

    // You can try adding a semi-colon on the end of the statement at Line 26, 
    // and see what happens. 
}