fn main() {
    let mut a = String::from("Owned string");
    let a_ref = &mut a;
    a_ref.push_str(" and owned string");
}
