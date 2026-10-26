use std::io;

fn main() {
    println!("Type a number to check if it is prime");

    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");

    let num : u64 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => 
            {
                eprintln!("Failed to parse number");
                return;
            }

    };

    let mut divisors = Vec::new();
    let mut prime = true; 

    if num == 0 {
        println!("Not prime!");
        return;
    } 
    
    if num == 1 {
        println!("Not prime!");
        return;
    }

    for i in 2..num {
         if num % i == 0 {
            divisors.push(i);
            prime = false;
         }
    }

    if prime {
        println!("Prime!");
    } else {
        println!("Not prime! Divisors of {:?}", divisors);
    };
    
}

/* 
make num prime by default (bool = true) 
for loop and divide num by 1..=num 
    if num divided by one of the numbers gets remainder 0 (num % .. == 0), print that it is not a prime
        *make loop check until very end and then print all divisors?
    stop loop once it runs through all numbers, have else statement to print that num is prime 

*/
