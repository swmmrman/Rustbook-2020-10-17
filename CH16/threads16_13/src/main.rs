use std::sync::{Arc,Mutex};
use std::thread;

fn check_prime(number: i32) -> bool {
    let mut prime = true;
    for i in 3..number+1 {
        if number / i == 0 {
            prime = false
        }
    }
    prime
}

fn main() {
    let pool = [1,2,3,4,5,6,7,8,9,10];
    let index = Arc::new(Mutex::new(0));
    //let primes = Mutex::new(vec![]);
    let mut threads = vec![];

    for _ in 0..10 {
        let index = Arc::clone(&index);
        let handle = thread::spawn(move || {
            let mut cur_index = index.lock().unwrap();
            let num = pool[*cur_index];
            if num % 2 != 0 {
                if check_prime(num) {
                    println!("{} is prime", num);
                }
            }
            *cur_index += 1;
        });
        threads.push(handle);
    }
}
