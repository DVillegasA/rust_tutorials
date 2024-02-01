use std::cmp::PartialOrd;

struct Point<T, U> {
    x: T,
    y: U
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }

    fn mixup<X1, Y1>(self, other: Point<X1, Y1>) -> Point<T, Y1> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }

    return largest;
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let integer = Point {x: 5, y: 10};
    let float = Point {x: 1.2, y: 4.5};
    let int_and_float = Point {x: 2, y: 5.3};

    println!("This point is an integer: x={}, y={}", integer.x(), integer.y());
    println!("This point is a float: x={}, y={}", float.x(), float.y());
    println!("This point is both an integer and a float: x={}, y={}", int_and_float.x(), int_and_float.y());

    println!("Distance from the origin for x={} and y={}: {}", float.x(), float.y(), float.distance_from_origin());

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}