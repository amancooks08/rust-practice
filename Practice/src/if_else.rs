pub fn run() {
    let num = 10;
    
    // Let's use the logic from functions module to check if a number is even or not.
    // We will use the if-else statement to check if the number is even or not.
    // The semantics around if-else are the same as in other languages.
    
    if num % 2 == 0 {
        println!("{} is even", num);
    } else {
        println!("{} is odd", num);
    }
}