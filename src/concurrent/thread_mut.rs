use std::sync::Arc;
use std::thread;
fn main() {
    let mut nums = Arc::new(vec![]);
    for n in 0..5 {
        let mut ns = nums.clone();
        thread::spawn(move || {
            nums.push(n);
        });
    }
}
