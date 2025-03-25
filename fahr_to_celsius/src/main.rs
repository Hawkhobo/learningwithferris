use std::io;

fn main() {
    println!("determine entered degree in fahrenheit or celsius");

    'end_prog: loop {
        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => { println!("re-enter: "); continue; }
        };

        loop {
            println!
            ("specify if this value is celsius (c) or fahrenheit (f): ");

            let mut deg = String::new();

            io::stdin()
                .read_line(&mut deg)
                .expect("Failed to read line");

            let deg: char = match deg.trim().parse() {
                Ok(degree) => degree,
                Err(_) => continue,
            };

            if deg == 'c' {
                println!("value in fahrenheit: {}", c_to_f(temp));
                break 'end_prog;
            }
            else if deg == 'f' {
                println!("value in celsius: {}", f_to_c(temp));
                break 'end_prog;
            }
            else {
                println!("Not valid input!");
                continue;
            }
        }
    }
}

fn c_to_f(temp: f64) -> f64 {
    (9.0 / 5.0)*temp + 32.0
}

fn f_to_c(temp: f64) -> f64 {
    (5.0 / 9.0) * (temp - 32.0)
}
