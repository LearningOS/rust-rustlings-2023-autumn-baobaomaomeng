// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.



use std::sync::mpsc;
use std::sync::{Arc,Mutex};
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q:Queue, tx: mpsc::Sender<u32>) -> () {
    let qc = Arc::new(Mutex::new(q));
    let txc = Arc::new(Mutex::new(tx));
    let qc1 = Arc::clone(&qc);
    let qc2 = Arc::clone(&qc);
    let txc1 = Arc::clone(&txc);
    let txc2 = Arc::clone(&txc);

    thread::spawn(move || {
        let q = qc1.lock().unwrap();    
        let t = txc1.lock().unwrap();
        for val in &q.first_half {
            println!("sending {:?}", val);
            t.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let q = qc2.lock().unwrap();
        let t = txc2.lock().unwrap();
        for val in &q.second_half {
            println!("sending {:?}", val);
            t.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    send_tx(queue, tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}
