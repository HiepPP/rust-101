use std::thread;

fn main() {
    thread::spawn(|| {
        println!("Thread");
        "Much concurrent, such now!".to_string()
    });
    print!("Hello");
}
