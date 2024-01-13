fn main() {
    let condition = true;
    let number = if condition {5} else {6};

    println!("The value of number is: {number}");

    if number % 4 == 0 {
        println!("The number is divisible by 4");
    } else if number % 3 == 0 {
        println!("The number is divisible by 3");
    } else if number % 2 == 0 {
        println!("The number is divisible by 2");
    } else {
        println!("The number is not divisible by 4, 3 or 2");
    }

    // If the value of an immutable variable gets assign once in the code
    // Rust allows us to define said variable like this:
    let x;
    x = 1;
    println!("The value of x is: {x}");
}
