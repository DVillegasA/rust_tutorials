struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}

fn main() {
    let mut user1 = build_user(
        String::from("someusername123"), 
        String::from("someone@example.com")
    );

    user1.email = String::from("anotheremail@example.com");

    if user1.active {
        user1.sign_in_count += 1;
        println!("Hello {}, you have logged in {} times", user1.username, user1.sign_in_count);
    } else {
        println!("Invalid User!");
    }

    let user2 = User {
        email: String::from("yetanotheremail@example.com"),
        ..user1
    };

    println!("Hello {}, you have logged in {} times", user2.email, user2.sign_in_count);
}
