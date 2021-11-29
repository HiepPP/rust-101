use std::rc::Rc;
use std::thread;

fn main() {
    let nums = Rc::new(vec![0, 1, 2, 3, 4]);
    let mut childs = vec![];
    for n in 0..5 {
        let ns = nums.clone();
        let c = thread::spawn(|| {
            //Rc<Vec<i32>>` cannot be shared between threads safely
            println!("{}", ns[n]);
        });
        childs.push(c);
    }

    for c in childs {
        c.join().unwrap();
    }
}
