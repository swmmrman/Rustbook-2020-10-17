fn main() {
    let mut number: i64 = 0x00000000000003F;

    while number & 0b10000000000000000000000000000000 == 0 {
        println!("{:064b}!", number);
        number -= 1;
        let wait = std::time::Duration::from_secs(1);
        std::thread::sleep(wait);
    }
    println!("Liftoff!!!");
}
