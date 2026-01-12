fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // note Rust will not automatically convert non-Boolean types to Boolean

    let condition = true;
    // note: these scalar values are expressions!
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
