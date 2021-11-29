use std::sync::mpsc;
use std::thread;

fn main() {
    let (sender, receiver) = mpsc::sync_channel(1);
    let sender_clone = sender.clone();

    let _ = sender.send(0);

    thread::spawn(move || {
        let _ = sender.send(1);
    });

    thread::spawn(move || {
        let _ = sender_clone.send(2);
    });

    println!("Received {} via the channel", receiver.recv().unwrap());
    println!("Received {} via the channel", receiver.recv().unwrap());
    println!("Received {} via the channel", receiver.recv().unwrap());
    println!("Received {:?} via the channel", receiver.recv());
}
