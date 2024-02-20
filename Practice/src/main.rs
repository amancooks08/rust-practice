mod data_types; // Importing the data_types module from data_types.rs file.
mod mutable; // Importing the mutable module from mutable.rs file.
mod arrays; // Importing the arrays module from arrays.rs file.
mod tuples; // Importing the tuples module from tuples.rs file.
mod strings; // Importing the strings module from strings.rs file.

// When you want to use multiple src files in a single project, 
// you can use mod to import them, just make sure they have public functions to call.
// by default functions are private in Rust.

// PS - Semi-colons are a thing in Rust :P

fn main() {
    // Calling the run function from data_types module.
    // You do it by <module-=name>::<function-name>;

    // data_types::run();
    // mutable::run();
    // arrays::run();
    // tuples::run();
    strings::run();
}
