fn main() {
    let _x = 2.0; // f64

    let _y: f32 = 3.0; // f32
    
    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // Results in -1
    
    // remainder
    let _remainder = 43 % 5;

    let _t = true;
    let _f: bool = false; // with explicit type annotation

    let _c = 'Z';
    let _z: char = 'Z'; // with explicit type annotation
    let _heart_eyed_cat = '😻';

    // optional type annotation. tuple type inference also works
    // distinction here is 1 would likely also be i32
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // example of destructuring a tuple
    let (_x, _y, _z) = tup;
    println!("The value of y is: {_y}");
    // example of indexing a tuple
    let _five_hundred = tup.0;
    let _six_point_four = tup.1;
    let _one = tup.2;

    // the tuple unit
    let _unit = ();

    // modify elements of a tuple
    let mut x: (i32, i32) = (1, 2);
    x.0 = 0;
    x.1 += 5;

    // simple arrays
    let _a = [1, 2, 3, 4, 5];
    let _month = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    // special initialization for array with same values
    let _a = [3; 5];
    // would be very useful in the case of...
    let mut _a = [false; 20];

    // accessing array elements
    let a = [1, 2, 3, 4, 5];
    let _first = a[0];
    let _second = a[1];

    use std::io;
    println!("Please enter an array index.");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
