#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }

}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(15 * scale),
        height: 50
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45
    };
    let sq = Rectangle::square(20);

    dbg!(&rect1);
    dbg!(&sq);
    println!("The area of the rectangle rect1 is {} pixels", rect1.area());

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let mut rect = Rectangle {
        width: 0,
        height: 0,
    };

    rect.set_width(2);
    println!("Area of rect: {}", rect.area());

    let other_rect = Rectangle {
        width: 1,
        height: 1,
    };

    let max_rect = rect.max(other_rect);
    println!("Area of max_rect: {}", max_rect.area());
}
