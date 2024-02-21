pub fn run() {

    /*
        Option is a type that represents the presence or absence of a value.
        Rust uses Option types in many places because it encodes the very 
        common scenario in which a value could be something or it could be nothing.

        Sometimes it's desirable to catch the failure of some parts of a program 
        instead of calling panic!; this can be accomplished using the Option enum.
        
        The Option<T> enum has two variants:

        --> None, to indicate failure or lack of value, and
        --> Some(value), a tuple struct that wraps a value with type T.

    */

    // Lets see how we can use Option in Rust.
    // We create two Option type variables, one with a value and one without a value.
    let divide_result1: Option<i32> = divide(10, 2);
    let _divide_result2: Option<i32> = divide(10, 20);

    // Now if we print the result we get the following output.
    println!("Result of division 1 is {:?}", divide_result1);

    // Something seems off, right?
    // We are getting the output as "Some(5)" instead of just "5".
    // Now there is a shorthand way to remove this using "unwrap" method.
    println!("Result of division 1 is {}", divide_result1.unwrap());
    // Got the output as "5" instead of "Some(5)"? BINGO!
    
    // println!("Result of division 2 is {}", _divide_result2.unwrap());   
    // Try removing the comment in the above Line, and see what happens? 
}

// First we define a function divide which basically divides two numbers.
fn divide(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        None
    } else {
        Some(dividend / divisor)
    }
}