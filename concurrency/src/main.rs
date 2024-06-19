use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = vec![
            String::from("Hello"),
            String::from("Hi!"),
            String::from("From"),
            String::from("Thread"),
        ];

        for i in val {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
    
}
