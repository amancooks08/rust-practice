use std::collections::HashMap;

pub fn run() {
    let mut scores = HashMap::new();

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