fn main() {
    // let mut vec: Vec<i32> = Vec::new();
    let mut vec = vec![1, 2, 3];
    let last = vec.pop();

    match last {
        Some(n) => println!("{n}"),
        None => println!("The vector was empty.")
    }
}
