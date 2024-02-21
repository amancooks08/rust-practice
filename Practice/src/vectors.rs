pub fn run() {
    // Vectors are re-sizable arrays.
    // They are useful when you want to store a list of items.
    // They are stored on the heap, not the stack.
    // A vector is represented using the Vec<T> type.
    // T is the type of the elements that the vector will store.
    // You can create an empty vector using the new function.
    let mut _vec: Vec<i32> = Vec::new();

    // You can also use the vec! macro to create a vector.
    let mut vec = vec![1, 2, 3];

    // You can add elements to a vector using the push method.
    vec.push(4);
    vec.push(5);
    vec.push(6);

    // You can also remove elements from a vector using the pop method.
    let _last = vec.pop();
    
    
    // You can access elements of a vector using indexing.
    let third: &i32 = &vec[2];
    println!("The third element is {}", third);

    // You can also use the get method to access elements of a vector.
    match vec.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // You can iterate over the elements of a vector using a for loop.
    for i in &vec {
        println!("{}", i);
    }

    // You can also iterate over the elements of a vector using the iter method.
    for i in vec.iter() {
        println!("{}", i);
    }

    // You can also iterate over the elements of a vector using the iter_mut method.
    for i in vec.iter_mut() {
        *i += 50;
    }

    for i in &vec {
        println!("{}", i);
    }
}

/*
    The standard library provides a number of useful datat types like Vectors.
    You can check it out at https://doc.rust-lang.org/rust-by-example/std.html
*/
