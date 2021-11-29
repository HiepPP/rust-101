use std::thread;

fn main() {
    let child = thread::spawn(|| {
        println!("Thread");
        String::from("Much concurrent, such now!")
    });
    print!("Hello");
    let value = child.join().expect("Failed joining child thread");
    println!("{}", value);
}
