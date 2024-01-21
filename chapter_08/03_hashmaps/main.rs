use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Red");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("Team {team_name} currently has {score} points!");

    for (k, v) in &scores {
        println!("Team {k} currently has {v} points!");
    }

    // For types that implement the Copy trait, HashMap makes a copy of their values
    // In case they don't implement it, the HashMap is going to take ownership of them
    let field_name = String::from("Favorite Color");
    let field_value = String::from("Blue");
    println!("My {field_name} is {field_value}");
    
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    
    // We can't use this print anymore 
    // because we moved them into the hash map
    // println!("My {field_name} is {field_value}");

    // If we insert a key that already existed in the hash map 
    // its old value get overwrited
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // If we want to insert only if the key doesn't already exist
    // we can use the entry command
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // another form of update is to manipulate the value 
    // that already exists in the hash map
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    // Entry::or_insert inserts the value only if their key doesn't exists, 
    // returning a mutable references to the inserted value, 
    // but in case they do it still returns the mutable reference
    for word in text.split_whitespace() {
        let count: &mut i32 = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}