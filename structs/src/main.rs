#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let rect = Rectangle{
        width:30,
        height:50,
    };
    println!(
        "The area of the rectangle is {} pixels squared.",
        area(&rect)
    );
    println!("rect is {:?}", rect);
}
