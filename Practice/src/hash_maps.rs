use std::collections::HashMap;

pub fn run() {

    /*
        Where vectors store values by an integer index, HashMaps store values by key.
        HashMap keys can be booleans, integers, strings, or any other type that 
        implements the Eq and Hash traits. 
        You can create a HashMap with a certain starting capacity 
        using HashMap::with_capacity(uint), 
        or use HashMap::new() to get a HashMap with a default initial capacity (recommended).
     */
    let mut scores = HashMap::new();

    /*
        Any type that implements the Eq and Hash traits can be a key in HashMap. 
        This includes:
            bool (though not very useful since there are only two possible keys)
            int, uint, and all variations thereof
            String and &str (protip: you can have a HashMap keyed by String and call .get() with an &str)
        
        ****
            Note that f32 and f64 do not implement Hash, likely 
            because floating-point precision errors would make using 
            them as hashmap keys horribly error-prone.
        ****
     */
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    match score {
        Some(s) => println!("Score of {} is {}", team_name, s),
        None => println!("No score for {}", team_name),
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(50);

    // The above code can be interpreted as:
    // If the key exists, return a mutable reference to the value
    // If the key does not exist, insert the key with the value 50

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    // lets count the number of times each word appears in the variable text
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
}