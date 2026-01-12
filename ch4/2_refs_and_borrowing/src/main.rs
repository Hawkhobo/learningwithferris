// Rust's system of referencing is really great for using memory safely when ownership (and
// deallocation) gets in the way
// References are non-owning pointers; they point to the pass-by-reference in the argument, not the
// data on the heap.

fn main() {
    let m1 = String::from("Hello");
    let m2 =  String::from("world");
    greet(&m1, &m2); // note the ampersands
    let s = format!("{} {}", m1, m2);
}

// the usual notion of deallocation still holds! ownership wasn't moved.
// so when this function ends, "Hello world" isn't deallocated.
fn greet(g1: &String, g2: &String) {
    println!("{} {}!", g1, g2);
}
