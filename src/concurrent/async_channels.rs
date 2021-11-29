use std::sync::mpsc::channel;
use std::thread;

//mpsc is multiple producer, single consumer
fn main() {
    //tx is the transmitter end, having type Sender<T>, implement Clone trait
    //rx is the receiver end, having type Receiver<T>, not implement Clone trait
    // name is convention and can name them anything
    let (tx, rx) = channel();
    let join_handle = thread::spawn(move || {
        // Keep receiving in a loop, until tx is dropped
        while let Ok(n) = rx.recv() {
            //Note: recv() always blocks
            println!("Received {}", n);
        }
    });
    for i in 0..10 {
        tx.send(i).unwrap();
    }
    join_handle.join().unwrap();
}
