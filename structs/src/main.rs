fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn main() {
    println!(
        "The area of the rectangle is {} pixels squared.",
        area((30,50))
    );
}
