fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn main() {
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} pixels squared.",
        area(width1, height1)
    );
}
