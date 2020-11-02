fn main() {
    let mut number: i64 = 0x7FFFFFFFFFFFFFFF;

    while !number != 0 {
        println!("{:064b}!", number);
        number -= 1;
        let wait = std::time::Duration::from_millis(1);
        std::thread::sleep(wait);
    }
    println!("Liftoff!!!");
}
