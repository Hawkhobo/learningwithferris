fn main() {
    loop {
        println!("again!");
        break;
    }

    let mut counter = 0;

    // if we used return instead of break, we'd exit the current function. 
    // Note return is just "counter * 2"
    // the semicolon for break is optional. this is the expression!
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");


    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);

        index += 1;
    }
    
    // a more efficient control flow, specifically for arrays
    for element in a {
        println!("The value is: {element}");
    }

    // even if you want to run code a certain number of times, most Rustaceans will use a for
    // loop! See LIFTOFF refactored.
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
