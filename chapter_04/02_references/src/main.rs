fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(&m1, &m2);
    let s = format!("{} {}", m1, m2);
    println!("Formated string: {}", s);

    // dereferencing examples
    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x);      // explicit dereference
    let x_abs2 = x.abs();           // implicit dereference
    assert_eq!(x_abs1, x_abs2);

    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r);     // explicit dereference
    let r_abs2 = r.abs();           // implicit dereference
    assert_eq!(r_abs1, r_abs2);

    let s = String::from("Hello");
    let s_len1 = str::len(&s);      // explicit dereference
    let s_len2 = s.len();           // implicit dereference
    assert_eq!(s_len1, s_len2);

    // vectors
    let mut v: Vec<i32> = vec![1, 2, 3];
    // let num: &i32 = &v[2];
    v.push(4);
    // cannot borrow `v` as mutable because it is also borrowed as immutable
    // println!("Third element  is {}", *num);
}

fn greet(g1: &String, g2: &String) {
    println!("{} {}!", g1, g2);
}
