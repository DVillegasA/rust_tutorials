use std::io;

fn main() {
    let x: u32 = 1_000;
    let y: u32 = "42".parse().expect("Not a number!");
    let z: f64 = 32.5;

    let result_int: u32 = x + y;
    let result_float = x as f64 + z;

    println!("x + y = {result_int}");
    println!("x + z = {result_float}");

    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    println!("-5 / 3 = {truncated}");

    // remainder
    let _remainder = 43 % 5;

    // boolean
    let _t = true;
    let _f = false;

    // character
    let _c: char = 'z';

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("Five Hundred: {five_hundred}");
    println!("Six Point Four: {six_point_four}");
    println!("One: {one}");

    let (x, y, z) = tup;

    println!("The value of x is {x}");
    println!("The value of y is {y}");
    println!("The value of z is {z}");

    // arrays
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let bunch_of_threes = [3; 5];

    println!("A bunch of Threes: {:?}", bunch_of_threes);

    println!("\nPlease enter an array index.");

    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index entered was not a number");

    let element = arr[index];

    println!("The value of the element at index {index} is: {element}");
}
