fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn main() {
    println!(
        "The area of the rectangle is {} pixels squared.",
        area(30,50)
    );
}
