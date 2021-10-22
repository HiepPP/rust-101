fn req_status() -> u32 {
    200
}

fn main() {
    let status = req_status();
    match status {
        200 => println!("success"),
        400 => println!("not found"),
        other => {
            println!("Request failed with code: {}", other);
        }
    }
}
