const DAYS: [&str; 12] = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
const GIFTS: [&str; 12] = ["And a partridge in a pear tree", "Two turtle doves", "Three french hens", "Four calling birds", "Five gold rings", "Six geese a-laying", "Seven swans a-swimming", "Eight maids a-milking", "Nine ladies dancing", "Ten lords a-leaping", "Eleven pipers piping", "Twelve drummers drumming"];

fn main() {

    for (i, d) in DAYS.iter().enumerate() {
        day_of_christmas(d, i);
    }
}

fn day_of_christmas(day: &str, mut index: usize) {
    println!("On the {day} day of Christmas my true love gave to me");

    // if it's the first day, print alternate partridge line
    if day == "first" {
        println!("A partridge in a pear tree.");
    }
    // otherwise, run algorithm normally
    else {
        while index > 0 {
            println!("{}", GIFTS[index]);
            index -= 1;
        }
        if index == 0 {
            println!("{}", GIFTS[index]);
        }
    }

}
