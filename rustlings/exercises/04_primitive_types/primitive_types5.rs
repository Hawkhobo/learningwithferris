fn main() {
    let cat = ("Furry McFurson", 3.5);

    let (name, age) = (cat.0, cat.1);

    println!("{name} is {age} years old");
}
