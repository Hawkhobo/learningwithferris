// rustc refuses to compile because it detects a possible data race
fn mutable_borrow(s1: &mut String, s2: &String) {
    s1.push_str(", world");
    println!("{s2}");
}

fn main() {
    let mut string1 = String::from("hello");

    mutable_borrow(&mut string1, &string1);
}
