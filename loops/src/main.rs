fn main() {
    let mut number = 0x000001F;

    while number & 0b1000000000000000 == 0 {
        println!("{:032b}!", number);
        number -= 1;
    }
    println!("Liftoff!!!");
}
