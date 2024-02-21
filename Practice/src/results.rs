// I'll be declaring an custom Error Enum inorder to use the Result type. 
#[derive(Debug)]
enum Error {
    DivisionByZero,
}

// We'll use the same divide function from the options module and see 
// how we can use the Result type in Rust.
fn divide(dividend: i32, divisor: i32) -> Result<i32, Error> {
    if divisor == 0 {
        Err(Error::DivisionByZero)
    } else {
        Ok(dividend / divisor)
    }
}

pub fn run(){

    let divide = divide(10, 2);
    // let res = divide.expect("Division by zero, we crashed!"); 

    match divide {
        Ok(result) => println!("The result is: {}", result),
        Err(e) => println!("Error: {:?}", e),
    }

    // Or you can use an if statement, to see the result 
    // for if statement, comment out the match block above.
    // if divide.is_ok() {
    //     println!("The result is: {}", divide.unwrap());
    // }

    // Or you could simply use the unwrap method to get the result.
    // println!("The result is: {}", divide.unwrap());

    // Now let's try some negative cases when there is an error.
    // You can return anything you want if the result is an error.
    // You could use the unwrap_or method to get the result.
    // To obtain the result you have to change te 2 to 0 on Line 19.
    // println!("Result is {:?}", divide.unwrap_or(100));

    // We'll also try to print the result of the error.
    // println!("Result is {:?}", res);
}