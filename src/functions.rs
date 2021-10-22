fn add(a: u64, b: u64) -> u64 {
    a + b
}

fn main() {
    let a: u64 = 17;
    let b = 3;
    let result = add(a, b);
    println!("Result {}", result);
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_add_pass(){
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_add_fail(){
        assert_ne!(add(1, 2), 4);
    }
}

