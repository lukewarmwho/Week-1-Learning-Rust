use std::io;

fn main() {
    println!("Please input a number");

    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");

    let mut num : u64 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => 
            {
                eprintln!("Failed to parse number");
                return;
            }

    };

    let mut count = 0;

    loop { 

        if num == 1 {
            break;
        }    

        if num % 2 == 0 {
            num = num / 2;  
        } else if num % 2 == 1 {
            num = (num * 3) + 1
        }

        count += 1;
        println!("{count}, {num}");
    }

    println!("Process complete! Steps: {count}");
    
}