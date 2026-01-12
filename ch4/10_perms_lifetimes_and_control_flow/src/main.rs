// capitalizes the first character in a vector of ASCII characters
fn ascii_capitalize(v: &mut Vec<char>) {

    // v loses W perms, keeps R perms
    let c = &v[0];

    if c.is_ascii_lowercase() {
        let up = c.to_ascii_uppercase(); // W returns to v
        // note up has no effect on ownership or perms!

        v[0] = up;
    } else { // Otherwise, W returns to v here
        println!("Already capitalized: {:?}", v);
    }
}

fn main() {
    let mut chars: Vec<char> = vec!['a', 'b', 'c'];
    ascii_capitalize(&mut chars);
    println!("New capitalized vector: {:?}", chars);
}
