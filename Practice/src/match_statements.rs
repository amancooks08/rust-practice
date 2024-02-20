pub fn run() {

    // In Rust we don't have switch statements, instead we have match.
    // The match keyword is used to compare a value against a series of patterns
    // and then execute code based on which pattern matches.

    // Let's take an example of a match statement to understand it better.
    let i = 3;

    match i {
        0 => println!("Zero"),
        1 | 2 => println!("One or Two"),
        3..=5 => println!("Three to Five"),
        _ => println!("Something else"),
    }
}