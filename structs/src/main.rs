#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
// You can have multiple impl blocks, valid but not needed
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle::square(40);  // Used to access class functions that don't need self.
    let rect3 = Rectangle::square(10);

    println!(
        "The area of the rectangle is {} square pixels. or {}",
        rect1.area(),
        area(&rect1) // The borrowing here lets us reuse the rect1 var throughout main (retaining ownership)
    );
    println!("rect1 is {:?}", rect1);
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}