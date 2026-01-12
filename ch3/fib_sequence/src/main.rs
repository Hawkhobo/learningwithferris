use std::io;

fn main() {
    println!("input value `n` to be fibonnacci sequence value: ");

    loop {
        let mut fib_value = String::new();

        io::stdin()
            .read_line(&mut fib_value)
            .expect("Error reading line.");

        let fib_value: i32 = match fib_value.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // return nth value
        println!("nth value of fibonnacci sequence is: {}", fib_sequence(fib_value));
        break;  
    }
}

fn fib_sequence(n: i32) -> i32 {
    if n == 0 || n == 1 {
        n
    }
    else {
        fib_sequence(n - 1) + fib_sequence(n - 2)
    }
}
