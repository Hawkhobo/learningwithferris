fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];

    let num: &i32 = &v[2];

    v.push(4);

    println!("Third element is {}", *num);
}

// This program does not compile because of the borrow checker.
// v cannot be mutated while the reference num is in use (num is borrowing)
// the more underlying issue: num could be invalidated by push. Rust catches this as a compiler
// error; you never have to find out about it during runtime of a Rust program.
