pub fn run() {

    // In Rust we have 3 types of loops:
    // Kindly uncomment the function call to see the output of a particular loop.
    // 1. loop
    // understanding_loop();

    // 2. while
    // understanding_while();

    // 3. for
    understanding_for();
}

fn understanding_loop() {
    // Rust provides a loop keyword to indicate an infinite loop.

    // The break statement can be used to exit a loop at anytime, 
    // whereas the continue statement can be used to skip the 
    // rest of the iteration and start a new one.

    // Below I have declared an unsigned 32-bit integer variable count 
    // and assigned it a value of 0. See how I did it?
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration, a 'continue' keyword like other languages.
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // Exit this loop
            break;
        }
    }

    // One of the uses of a loop is to retry an operation until it succeeds. 
    // If the operation returns a value though, you might need to pass it 
    // to the rest of the code: put it after the break, and it will be returned
    // by the loop expression...

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    // We can use the assert_eq! macro to check if the result is 20.
    // An assertion will panic if the result is not 20.
    // An assertion compares the result with the expected value and
    // panics if the two values are not equal, with the 
    // message provided in the third arguement.
    assert_eq!(result, 20, "The result is not 20");
    
    // Try changing the value to something other than 20 in Line 68, and see what happens.
    // Assertions are generally used when you are performing tests.
}

fn understanding_while() {

    //The 'while' keyword can be used to run a loop while a condition is true.
    // Let's write the infamous FizzBuzz using a while loop.

    // A counter variable
    let mut n = 1;

    // Loop while `n` is less than 101
    while n < 21 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // Increment counter
        n += 1;
    }
}

fn understanding_for() {
    // The for loop is used to loop over a range of values.
    fizzbuzz_using_range();

    // The for loop can also be used to iterate over a collection.
    fizzbuzz_using_iter();
}

fn fizzbuzz_using_range() {
    // The for in construct can be used to iterate through an Iterator. 
    // One of the easiest ways to create an iterator is to use the range notation a..b. 
    // This yields values from a (inclusive) to b (exclusive) in steps of one.

    // Let's write FizzBuzz using for instead of while.

    for n in 1..21 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // Alternatively, a..=b can be used for a range that is inclusive on both ends. 
}

fn fizzbuzz_using_iter() {
    // The range notation is just a shortcut for creating an iterator.
    // The for in construct is able to interact with an Iterator in several ways. As discussed in the section on the Iterator trait, by default the for loop will apply the into_iter function to the collection. However, this is not the only means of converting collections into iterators.

    // into_iter, iter and iter_mut all handle the conversion of a collection 
    // into an iterator in different ways, by providing different views on 
    // the data within..

    let names = vec!["Bob", "Frank", "Lebron"];

    // iter - This borrows each element of the collection through each iteration. 
    // Thus leaving the collection untouched and available for reuse after the loop.
    for name in names.iter() {
        if let &"Lebron" = name {
        // TODO ^ Try deleting the & and matching just "Lebron"
            println!("There is a rustacean among us!");
        } else {
            println!("Hello {}", name);
        }
    }
    
    println!("names: {:?}", names);

    // into_iter - This consumes the collection so that on each iteration 
    // the exact data is provided. Once the collection has been consumed 
    // it is no longer available for reuse as it has been 'moved' within the loop.

    for name in names.into_iter() {
        if let "Lebron" = name {
            // TODO ^ Try deleting the & and matching just "Lebron"
            println!("There is a rustacean among us!");
        } else {
            println!("Hello {}", name);
        }
    }
    
    // println!("names: {:?}", names);
    // TODO ^ Un-comment this line and see what happens.

    let mut names = vec!["Bob", "Frank", "Lebron"];
    // iter_mut - This mutably borrows each element of the collection, 
    // allowing for the collection to be modified in place.
    for name in names.iter_mut() {
        if &"Lebron" == name {
            println!("There is a rustacean among us!");
        }
        else {
            println!("Hello {}", name);
        }
    }

    println!("names: {:?}", names);
}
