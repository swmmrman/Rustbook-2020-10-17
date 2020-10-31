fn main() {
    let mut number = 0x000000F;

    while number & 0b1000000000000000 == 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("Liftoff!!!");
}
