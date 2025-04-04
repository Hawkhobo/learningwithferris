fn main() {
    println!("Hello, world!");

    another_function(5);

    print_labeled_measurement(5, 'h');

    // important distinction: expressions don't end in semicolons
    let _y = {
        let x = 3;
        x + 1
    };
    // ending with a semicolon at x + 1 would not return a value
    // why? Instead of using a `return` reserved keyword, like in C, we instead just have
    // expressions WITHOUT semicolons! Pretty neat.
    
    let mut x = five();
    println!("The value of x is: {x}");

    x = plus_one(5);
    println!("The value of x is: {x}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
