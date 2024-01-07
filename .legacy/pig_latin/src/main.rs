use std::io;

fn main() {
    let vowels = ['a','e','i','o','u'];
    let mut input = String::new();
    let output: String;

    println!("Please input a word.");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim().to_lowercase();

    if vowels.iter().any(|&v| input.starts_with(v)) {
        let input_lowered = &input.to_lowercase();
        output = format!("{input_lowered}-hay");
    } else {
        let first_char = &input[0..1].to_lowercase();
        let remainer_string = &input[1..].to_lowercase();
        output = format!("{remainer_string}-{first_char}ay");
    }
    println!("The word {input} in pig latin turns into: {output}");
}
