use std::sync::{Arc,Mutex};
use std::thread;

fn check_prime(number: i32) -> bool {
    let mut prime = true;
    for i in 3..number {
        if number % i == 0 {
            prime = false
        }
    }
    prime
}

fn main() {
    let max = 4000;
    let pool: Arc<Vec<i32>> = Arc::new((1..max).collect());
    let index = Arc::new(Mutex::new(0));
    //let primes = Mutex::new(vec![]);
    let mut threads = vec![];

    for _ in 0..max-1 {
        let index = Arc::clone(&index);
        let pool = Arc::clone(&pool);
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
    for thread in threads:
        thread.join();
    }
}
