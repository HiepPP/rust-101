fn main() {
    // let _my_result: Result<_, ()> = Ok(64);
    let my_result = Ok::<_, ()>(64);

    let my_error = Err::<(), f32>(32.3);
    let other_error: Result<bool, String> = Err("Error message".to_string());
}
