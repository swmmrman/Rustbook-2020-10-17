fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("The number was divisible by 4.");
    }
    else if number % 3 == 0 {
        println!("The number was divisible by 3.");
    }
    else if number % 2 == 0 {
        println!("The number was divisible by 2.");
    }
    else {
        println!("The number was not divisible by 4,3, or 2.");
    }
}
