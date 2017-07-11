use std::sync::mpsc;
use std::time::Duration;
use std::thread;
use std::sync::{Mutex, Arc};

fn main() {
    let (tx, rx) = mpsc::channel();
    let counter = Arc::new(Mutex::new(0));
    let producer_counter = counter.clone();
    let receiver_counter = counter.clone();

    let mut handles = vec![
        thread::spawn(move || producer(tx,producer_counter) ),
        thread::spawn(move || receiver(rx, receiver_counter) )
        ];

    for handle in handles {
        handle.join();
    }
}

fn producer(tx : mpsc::Sender<String>, counter : Arc<Mutex<i32>>) {
    while *counter.lock().unwrap() < 10 {
        for val in vec![
           String::from("Thank"),
           String::from("you"),
           String::from("for"),
            String::from("attending"),
            String::from("Xconf"),
        ] {
            tx.send(val).unwrap();
            thread::sleep(Duration::new(1, 0));
        }
    }
}

fn receiver(rx : mpsc::Receiver<String>, counter : Arc<Mutex<i32>>) {
    for received in &rx {
        println!("Got: {}", received);
        if received == "Xconf" {
            *counter.lock().unwrap() += 1;
        }
    }
}
