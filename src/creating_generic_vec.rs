fn main() {
    // provide a type
    let v1: Vec<u8> = Vec::new();

    // or calling method
    let mut v2 = Vec::new();
    v2.push(1);

    // or using turbofish
    let v3 = Vec::<u8>::new(); //not so readable
}
