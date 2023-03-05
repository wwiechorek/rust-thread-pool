use std::sync::{mpsc::channel, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = channel();
    let rx = Arc::new(Mutex::new(rx));

    for i in 1..=2 {
        let rx = Arc::clone(&rx);
        thread::spawn(move || loop {    
            thread::sleep(Duration::from_secs(i));
            let msg = rx.lock().unwrap().recv().unwrap();
            println!("Consumer {} received message: {}", i, msg);
        });
    }

    for i in 0..10 {
        let msg = format!("Message {}", i);
        tx.send(msg).unwrap();
    }

    loop{}
}