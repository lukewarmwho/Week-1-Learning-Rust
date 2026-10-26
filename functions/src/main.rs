use std::io;

fn main() {

    another_function();
}

fn another_function() {
    println!("What is the magnitude?");

    let mut mag = String::new();

    io::stdin()
        .read_line(&mut mag)
        .expect("Failed to read line");

    let mag: i32 = match mag.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Failed to parse magnitude");
            return;
        }
    };

    println!("What is the measurement?");

    let mut measure = String::new();

    io::stdin()
        .read_line(&mut measure)
        .expect("Failed to read line");

    let measure = measure.trim();

    println!("{mag}{measure}");
}
