mod data_types; // Importing the data_types module from data_types.rs file.

// When you want to use multiple src files in a single project, 
// you can use mod to import them, just make sure they have public functions to call.
// by default functions are private in Rust.

// PS - Semi-colons are a thing in Rust :P

fn main() {
    // Calling the run function from data_types module.
    // You do it by <module-=name>::<function-name>;
    data_types::run();
}
