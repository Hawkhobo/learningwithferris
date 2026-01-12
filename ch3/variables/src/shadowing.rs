fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // variable shadowing can easily inherit new data types
    let spaces = "   ";
    let spaces = spaces.len();

    // this results in a compile-time error
    let mut spaces = "   ";
    spaces = spaces.len();
    // this is NOT shadowing, and you can't mutate a variable's type!
    // Just the values in the assigned type.
}
