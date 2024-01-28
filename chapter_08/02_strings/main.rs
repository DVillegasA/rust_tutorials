fn main() {
    let data = "initial contents";
    let s = data.to_string();

    println!("{s}");
    
    // the method also works in literals directly:
    let s = "initial contents yet again".to_string();

    println!("{s}");

    // the method push_str takes a string slice and appends it to a string literal
    // therefore it doesn't take ownership of the pushed data, it simply copies it
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);

    println!("s2 is {s2}");
    println!("s1 is {s1}");

    // in contrast the method push only appends one character at a time
    let mut s3 = String::from("lo");
    s3.push('l');

    println!("s3 is {s3}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // the add method on strings takes ownership of the first string
    // it then appends a string slice copy of the second string
    let s3 = s1 + &s2;

    println!("{s3}");
    println!("{s2}");
    // println!("{s1}");

    // in contrast the format! method uses references for each of its strings
    // so it never takes ownership away from the original variables.
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");
    println!("{s1}");
    println!("{s2}");
    println!("{s3}");

    // each character in "Зд" is made up off two bytes, so when we iterate over it
    // we would get different iterators with different lengths depending if we
    // are iterating over the characters that make the word "Зд"
    for c in "Зд".chars() {
        println!("{c}");
    }

    // or over the bytes that make the same word.
    for b in "Зд".bytes() {
        println!("{b}");
    }
}