pub fn run() {

    // Let's see if we can declare a tuple in Rust.

    let person: (&str, bool, i8) = ("John", true, 25);
    // The type of the tuple is (&str, &str, i8).
    // Yes the tuple can have values of different data types.
    
    // You can access the elements of the tuple using the index of the element.
    // The index of the tuple starts from 0.
    // We us the dot(.) operator to access the elements of the tuple.
    println!("person.0: {}", person.0);
    println!("person.1: {}", person.1);
    println!("person.2: {}", person.2);
    println!("person: {:?}", person);
   
    // You can also destructure the tuple to get the values of the elements.
    // You can use the let keyword to destructure the tuple.
    let (name, is_adult, age) = person;
    println!("name: {}", name);
    println!("is_adult: {}", is_adult);
    println!("age: {}", age);
}