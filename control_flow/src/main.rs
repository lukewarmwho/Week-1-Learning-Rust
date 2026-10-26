use std::io;


fn main() {

    /*println!("Please input a number:");
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: i32 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Failed to parse number");
            return;
        }
    };

    if number != 0 {
        println!("number was something other than zero");
    } */

    // the following method is very slow and inefficient
    /*if number < 3{
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } 
    if number % 3 == 0 {
        println!("number is divisible by 3");
    } 
    if number % 2 == 0 {
        println!("number is divisible by 2");
    } 
    if number % 5 == 0 {
        println!("number is divisible by 5");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }*/

    /*let divisors = [2,3,4,5,6,7,8,9,10];

    for divisor in divisors {
        if number % divisor == 0 {
            println!("number is divisible by {divisor}");
        }
    }

    let condition = true;
    let num = if condition { 5 } else { 6 };

    println!("The value of num is: {num}"); */
/*use std::thread;
use std::time::Duration;

    let mut count = 0;

    let result = loop {
        count += 1;
        println!("count is: {count}");
        
        thread::sleep(Duration::from_secs(1));

        if count == 10 {
            break;
        }
    }; */
    
 /*     let mut a = 0;
    'counting_up: loop {
        println!("count = {a}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if a == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        a += 1;
    }
    println!("The result is: {a}");
 */

    /*for number in (1..4).rev() {
        println!("{number}!");
    }

    println!("LIFTOFF!!!");

    let mut a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    } */

    println!("Please input 'f' to convert to Fahrenheit or 'c' to convert to Celsius:");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice = choice.trim().to_lowercase();

    if choice == "f" {
        cel_to_fahr();

    } else if choice == "c" {
        fahr_to_cel();
    } else {
        eprintln!("Invalid choice");
    }

}

fn cel_to_fahr() {
    println!("Please input the temperature in Celsius");

    let mut cel  = String::new();

    io::stdin()
        .read_line(&mut cel)
        .expect("Failed to read line");

    let cel : f64 = match cel.trim().parse() {
            Ok(num) => num,
            Err(_) => 
            {
                eprintln!("Failed to parse number");
                return;
            },
        };
    
    let fahr = (cel * 9.0 / 5.0) + 32.0;
    println!("{fahr} degrees Fahrenheit");
}

fn fahr_to_cel () {
    println!("Please input the temperature in Fahrenheit");

    let mut fahr = String::new();

    io::stdin()
        .read_line(&mut fahr)
        .expect("Failed to read line");

    let fahr : f64 = match fahr.trim().parse() {
            Ok(num) => num,
            Err(_) => 
            {
                eprintln!("Failed to parse number");
                return;
            },
        };
    
    let cel = (fahr - 32.0) * 5.0 / 9.0;
    println!("{cel} degrees Celsius");

}