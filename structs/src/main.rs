#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn double_sides(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }
    fn multiply_sides(&mut self, multi: u32) {
        self.width *= multi;
        self.height *= multi;
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut rect1 = Rectangle {
        width:30,
        height:50,
    };
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
    rect1.multiply_sides(2);
    println!("Can a doubled rect1 hold rect3? {}", rect1.can_hold(&rect3));
    rect1.double_sides();
    println!(
        "What if we double again? {},  The area is now {}",
        rect1.can_hold(&rect3),
        rect1.area(),
    );
    let square = Rectangle::square(60);
    println!("Can the square hold rect2? {}", square.can_hold(&rect2));
}
