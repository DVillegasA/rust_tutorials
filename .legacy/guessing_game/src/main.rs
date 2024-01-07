use std::{cmp::Ordering, io};
use rand::Rng;
mod guess;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");
    
        let mut guess_aux = String::new();
    
        io::stdin()
            .read_line(&mut guess_aux)
            .expect("Failed to read line");
    
        let guess_aux: i32 = match guess_aux.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a number, try again!");
                continue;
            },
        };

        let guess = guess::Guess::new(guess_aux);
    
        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
