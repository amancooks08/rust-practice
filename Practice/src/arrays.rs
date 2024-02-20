pub fn run() {

    // Let's see how to declare an array in Rust.
    // You declare an array using the 'let' keyword, followed by the name of the array,
    // then a colon, then the type of the array, then the length of the array,
    // and then the values of the array.
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("numbers: {:?}", numbers);
    
    // You can also declare an array without specifying the type and length of the array.
    // Rust can infer the type and length of the array from the values you provide.
    let numbers_inferred = [1, 2, 3, 4];
    println!("numbers_inferred: {:?}", numbers_inferred);
    
    // You can also declare an array with the same value for all the elements of the array.
    let same_numbers = [3; 5]; // This will create an array of length 5 with all the elements as 3.
    println!("same_numbers: {:?}", same_numbers);

    // You can access the elements of the array using the index of the element.
    // The index of the array starts from 0.
    println!("numbers[0]: {}", numbers[0]);
    println!("numbers_inferred[1]: {}", numbers_inferred[1]);
    println!("same_numbers[2]: {}", same_numbers[2]);

    // You can also change the value of an element of the array using the index of the element.
    // The value of the array should be mutable to change the value of the element.
    let mut mutable_numbers = [1, 2, 3, 4, 5];
    println!("mutable_numbers: {:?}", mutable_numbers);

    // If you've noticed I used the mut keyword to make the variable mutable.
    // Keep an eye out for that!

    mutable_numbers[0] = 10;
    println!("mutable_numbers: {:?}", mutable_numbers);
    // Now the value of the first element of the array is changed to 10.

    // You can also access the length of the array using the len() method.
    println!("Length of numbers: {}", numbers.len());
    
    // You can also access the first and last elements of the array 
    // using the first() and last() methods respectively.
    println!("Last element of numbers: {:?}", numbers.last());
    println!("First element of numbers: {:?}", numbers.first());

    // Did you find something annoying when you tried to print the first and last elements of the array?
    // You got the output as Some(1) and Some(5) right?
    // This is because the first() and last() methods return an Option type.
    // We'll learn about the Option type in the next section.
    // For now, you can use the unwrap() method to get the value of the Option type.
    // Uncomment the below line to see the output.
    // println!("Last element of numbers: {:?}", numbers.last().unwrap());


    // You can also slice the array to get a part of the array.
    // You can slice the array using the index of the start and end of the slice.
    // The start index is INCLUSIVE and the end index is EXCLUSIVE.
    // You have to use the range operator '..' to slice the array.

    let slice = &numbers[1..4]; // This will slice the array from index 1 to 3.
    println!("slice: {:?}", slice);
    // The output will be [2, 3, 4].

    // You can also slice the array from the start to a specific index.
    let slice_start = &numbers[..3]; // This will slice the array from the start to index 2.
    println!("slice_start: {:?}", slice_start);
    // The output will be [1, 2, 3].

    // You can also slice the array from a specific index to the end.
    let slice_end = &numbers[2..]; // This will slice the array from index 2 to the end.
    println!("slice_end: {:?}", slice_end);
    // The output will be [3, 4, 5].

    // You can also slice the array to get the whole array.
    let slice_whole = &numbers[..]; // This will slice the whole array.
    println!("slice_whole: {:?}", slice_whole);
    // The output will be [1, 2, 3, 4, 5].
}