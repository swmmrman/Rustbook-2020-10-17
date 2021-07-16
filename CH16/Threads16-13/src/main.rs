use std::sync::Mutex;
use std::thread;

fn main() {
    let pool = Mutex::new([1,2,3,4,5,6,7,8,9,0]);
    let index = Mutex::new(0);
    let prims = Mutex::new(vec![]);
    let mut threads = vec![];
    
}
