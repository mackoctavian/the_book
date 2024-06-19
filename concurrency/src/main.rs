use std::sync::{mpsc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
    
}
