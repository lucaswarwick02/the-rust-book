#[derive(Debug)]
struct Rectangle {
    width: usize,
    height: usize,
}

impl Rectangle {  // We want to add something, within the context of Rectangle (method syntax)
    fn area(&self) -> usize {
        return self.width * self.height;
    }

    // Each of these functions are called 'associated functions'
    fn width(&self) -> bool {
        return self.width > 0;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }

    // This can be called with Rectangle::square(3)
    fn square(size: usize) -> Self {
        return Self {  // Self is an alias for the type (Rectangle)
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The are of the rectangle is {} square pixels", rect1.area());

    if rect1.width() {
        println!("TThe rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(25);
    println!("Square is width={}, height={}.", square.width, square.height);
}