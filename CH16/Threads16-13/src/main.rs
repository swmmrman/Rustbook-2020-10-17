use std::sync::Mutex;
use std::thread;

fn main() {
    let pool = [1,2,3,4,5,6,7,8,9,10];
    let index = Mutex::new(0);
    let prims = Mutex::new(vec![]);
    let mut threads = vec![];

    for i in 0..10 {
        let handle = thread::spawn(move || {
            let mut curIndex = index.lock().unwrap();
            
            *curIndex++;

        })
    }
}
