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
    // fn ca
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
}
