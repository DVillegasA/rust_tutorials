/*
&i32        -> a reference
&'a i32     -> a reference with an explicit lifetime
&'a mut i32 -> a mutable reference with an explicit lifetime
*/

use std::fmt::Display;

fn lifetime_example(){
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str, 
    y: &'a str, 
    ann: T
) -> &'a str 
where 
    T: Display 
{
    println!("Announcement! {}", ann);

    if x.len() > y.len() {
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
        let result = longest_with_an_announcement(string1.as_str(), string2, "THIS IS IMPORTANT!!!");
        println!("The longest string is {}", result);
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Couldn't find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    let part: &str = i.announce_and_return_part("This is important");
    println!("Important excerpt: {:?}", part);
    println!("Current level: {}", i.level());
}