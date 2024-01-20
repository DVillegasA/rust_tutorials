fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i+1),
        None => None
    }
}

fn main() {
    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);
}
