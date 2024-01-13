const SECONDS_IN_AN_HOUR: u32 = 60 * 60;
const SECONDS_IN_A_DAY: u32 = SECONDS_IN_AN_HOUR * 24;
const PI: f32 = 3.14159265359;

fn main() {
    // let x = 5;
    // let x = x + 1;

    let mut x = 1;

    {
        // let x = x * 2;
        let mut x = x;
        x += 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}\n");
    
    println!("There's {SECONDS_IN_AN_HOUR} seconds in an hour.");
    println!("There's {SECONDS_IN_A_DAY} seconds in a day.");
    println!("The value of pi is: {PI}");
}
