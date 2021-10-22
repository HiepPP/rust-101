fn increase_by(mut val: u32, how_much: u32) -> u32 {
    val += how_much;
    val
}

fn main() {
    let score = 2048;
    increase_by(score, 30);
}

#[test]
fn test_increase_by() {
    assert_eq!(increase_by(2048, 30), 2078);
}
