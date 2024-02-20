pub fn run() {
    // Primitive string
    // You can also declare a string using the 'let' keyword, followed by the 
    // name of the string, then a colon, then the type of the string, 
    // then the value of the string.

    let mut hello: String = String::from("Hello");
    // This is called borrowing. We are borrowing the value of
    // the string 'Hello' and assigning it to the variable 'hello'.

    // String is the dynamic heap string type, you use it when you need to own or modify your string data.

    let _world: &str = "World";
    // This is a string slice. String slices are immutable.
    // &str is an immutable sequence of UTF-8 bytes of dynamic length somewhere in memory.

    /* 
        Note: In rust, you can use an underscore to name unused variables.
        This is a good practice to follow, as it makes your code more readable.
    */

    // In summary, use String if you need owned string data (like passing 
    // strings to other threads, or building them at runtime), 
    // and use &str if you only need a view of a string.

    // Get length
    println!("Length: {}", hello.len());

    // Push char
    hello.push('W');

    // Push string
    hello.push_str("orld!");

    // Check if empty
    println!("Is Empty: {}", hello.is_empty());

    // Contains
    println!("Contains 'World' {}", hello.contains("World"));

    // Replace
    println!("Replace: {}", hello.replace("World", "There"));

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Capacity in bytes, by default it's 20 bytes.
    println!("Capacity: {}", hello.capacity());

    // But you can create string with custom capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);
    println!("{}", s.capacity());
}