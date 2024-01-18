struct AlwaysEqual;

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1
    }
}

fn main() {
    let user1 = build_user(String::from("First User"), String::from("fist_email@gmail.com"));

    let user2 = User {
        email: String::from("second_email@gmail.com"),
        ..user1
    };
    
    if user2.active {
        println!("The user {} with email {} has sign in {} times.", user2.username, user2.email, user2.sign_in_count);
    } else {
        println!("The user {} is currently innactive.", user2.username)
    }

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    let _subject = AlwaysEqual;

    let mut p = Point(0, 0, 0);
    let x = &mut p.0;
    *x += 1;

    println!("{}, {}, {}", p.0, p.1, p.2);
}
