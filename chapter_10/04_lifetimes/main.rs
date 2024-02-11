/*
&i32        -> a reference
&'a i32     -> a reference with an explicit lifetime
&'a mut i32 -> a mutable reference with an explicit lifetime
*/

fn lifetime_example(){
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len(){
        x
    } else {
        y
    }
}

fn main(){
    lifetime_example();
    let string1 = String::from("long string is long");
    {
        let string2 = "xyz";
        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }
}