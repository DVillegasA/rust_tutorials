use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file(filename: String) -> Result<String, io::Error> {
    let mut username = String::new();

    // First option:
    // let mut username_file = File::open(filename)?;
    // username_file.read_to_string(&mut username)?;

    // Secod option:
    File::open(filename)?.read_to_string(&mut username)?;
    
    Ok(username)
}

fn main() {
    let _greeting_file = File::open("hello.txt").expect("hello.txt should be included in this project");
    let _username = read_username_from_file(String::from("user.txt")).unwrap();
}