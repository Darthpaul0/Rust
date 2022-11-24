// https://doc.rust-lang.org/book/ch05-00-structs.html

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// define implementations for Rectangle structure
impl Rectangle {
    // calculates rectangle's area
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // check if width is not 0
    fn width(&self) -> bool {
        self.width > 0
    }
    // check if a rectangle fits in this rectangle
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // creates a square
    fn square(size: u32) -> Self {
        Self {
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
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    let sq = Rectangle::square(3);
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("rect1 is {:#?}", rect1);
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("{:#?}",sq);
}
