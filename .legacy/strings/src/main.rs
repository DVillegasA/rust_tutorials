fn main() {
    let s1 = String::from("Tic");
    let s2 = String::from("Tac");
    let s3 = String::from("Toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");
}
