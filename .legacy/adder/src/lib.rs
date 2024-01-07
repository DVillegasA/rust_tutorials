pub struct Rectangle {
    width: u32,
    height: u32,
}

pub struct Guess {
    value: i32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.", value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.", value);
        }

        Guess { value }
    }
}

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub fn greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;
    static LARGER: Rectangle = Rectangle {
        width: 8,
        height: 7,
    };
    static SMALLER: Rectangle = Rectangle {
        width: 5,
        height: 1,
    };

    #[test]
    fn larger_can_hold_smaller() {
        assert!(LARGER.can_hold(&SMALLER));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        assert!(!SMALLER.can_hold(&LARGER));
    }

    #[test]
    #[ignore]
    fn greeting_contains_carol() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was '{}'",
            result
        );
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
