use std::thread;
use std::time::Duration;

fn main() {
    let th = thread::spawn(|| {
        for i in 0..100 {
            println!("Printing the number {} from the child thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in  0..50 {
        println!("Print the number {} from the parent thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    th.join().unwrap();
}
