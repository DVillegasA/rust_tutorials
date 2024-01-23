use std::io;

fn main() {
    let vowels = ['a', 'e', 'i' , 'o', 'u'];
    let mut input_string = String::new();

    println!("Please input your word.");
    io::stdin().read_line(&mut input_string).expect("Failed to read line");

    let input_string = input_string.trim().to_lowercase();
    let chars = &mut input_string.chars();
    let output: String;
    
    if let Some(first) = chars.next() {
        if vowels.contains(&first) {
            output = format!("{input_string}-hay");
        } else {
            output = format!("{}-{first}ay", chars.as_str());
        }
    } else {
        output = String::from("ERROR: No word was provided");
    }
    
    println!("{output}");
}
