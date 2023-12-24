use std::{sync::{Arc, Mutex}};
use std::thread::spawn;

fn main() {
    let counter = Arc::new(Mutex::new(5));
    let mut thread_handles = Vec::new();
    for _ in 0..8 {
        let counter = Arc::clone(&counter);
        let handle = spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        thread_handles.push(handle);
    }
    for handle in thread_handles {
        handle.join().unwrap();
    }
    println!("{}", counter.lock().unwrap());
}