// Rust's automated memory management means that deallocation can be unwieldy
// We can return last ownership as pointers, to be able to use these values
// and all the while, deallocation and memory safety is guaranteed!
// the issue? this solution is a bit verbose isn't it? Rust can be more precise

fn main() {
    let m1 = String::from("Hello");
    let m2 =  String::from("world");
    let (m1_again, m2_again) = greet(m1, m2);
    let s = format!("{} {}", m1_again, m2_again);
}

fn greet(g1: String, g2: String) -> (String, String) {
    println!("{} {}!", g1, g2);
    (g1, g2)
}
