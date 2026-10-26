use std::io;

fn main() {
    println!("Please input what nth Fibonacci number you want to calculate:");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n : u64 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => 
            {
                eprintln!("Failed to parse number");
                return;
            }
    };

    let suffix = if n % 100 == 11 || n % 100 == 12 || n % 100 == 13 {
        "th"
    } else {
        match n % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        }

    };

    let result = fibn(n);
    println!("The {n}{suffix} Fibonacci number is {result}!");
}

fn fibn (n: u64) -> u64 {

    if n == 0 {
        return 0;
    } 
    if n == 1 {
        return 1;
    }


    let mut a = 0;
    let mut b = 1;

    for _ in 2..=n {
        let next_num = a + b;
        a = b;
        b = next_num;

    }

    b

}
