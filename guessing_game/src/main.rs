use std::cmp::Ordering;
use std::io;

use rand::prelude::*;

fn main() {
    println!("Guess the number!");

    //generates a random number between 1 and 100 and binds it to the variable secret_number, which is immutable by default
    let secret_number: u32 = rand::rng().random_range(1..=100);

    //println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess:");

         // creates a mutable variables (meaning it can change) and binds it to a new instance of a String
        let mut guess = String::new();

      /*in the first line we called from the standard library with "use std::io". 
      Now we call the "stdin" function from the "io" module, which allows us to handle user inputs
       Note that we can still use this function WITHOUT calling from the standard library by using std::io::Stdin*/
        io::stdin()
         .read_line(&mut guess)
         .expect("Failed to read line");

      /*shadowing the guess variable to convert it from a String to an unsigned 32-bit integer*/
      let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

       println!("You guessed: {guess}"); 

     match guess.cmp(&secret_number) {
           Ordering::Less => println!("Too small!"),
         Ordering::Greater => println!("Too big!"),
          Ordering::Equal => {
                println!("You win!");
                println!("The secret number was: {secret_number}");
                break;
            }
        }  
    }
}
