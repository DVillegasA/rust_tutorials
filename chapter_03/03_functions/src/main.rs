// Statements are instructions that perform some action and do not return a value.
// Expressions evaluate to a resultant value (they return something)

fn main() {
    let x = five();
    
    let y = {
        let x = 3;
        // by not adding a ; at the end of this line we turn it into an expression
        x + 1
    };
    
    another_function(x);
    println!("The value of y is: {y}");
    println!("The value of y + 1 is: {}", plus_one(y));
    print_labeled_measurement(x, 'h');
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}