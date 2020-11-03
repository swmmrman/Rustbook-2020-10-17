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
}

fn main() {
    let mut rect = Rectangle{
        width:30,
        height:50,
    };

}
