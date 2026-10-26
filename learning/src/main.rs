fn main() {
    println!("Hello, world!");
    println!("I'm a Rustacean!");
    let x = 1  + 9  - 8 ;
    println!("x = {}", x);
    println!("{} days", 31);

    /*let alice = "David";*/
    println!("{alice}, this is {0}. {0}, this is {alice}", "Bob", alice = "David");
    println!("{subject} {verb} {object}",
             object = "the lazy dog",
             subject = "the quick brown fox",
             verb = "jumps over");
    println!("{alice} {placeholder} {bob}, {bob} {placeholder} {alice}",
             alice = "Alice",
             placeholder = "does not like",
             bob = "Bob");
    second();
    
}
fn second() {
    println! ("Base 10: {}", 69420);
    println! ("Base 2 (binary): {:b}", 69420);
    println! ("Base 8 (octal): {:o}", 69420);
    println! ("Base 16 (hexadecimal): {:x}", 69420);
    println! ("Base 16 (hexadecimal, uppercase): {:X}", 69420);
    println! ("{number:>5}", number = 1);
    println! ("{number:0>5}", number = 1);
    println! ("{number:0<5}", number = 1);
    println! ("{number:0^5}", number = 1);
    println! ("{number:0^width$}", number = 1, width =10);
    println! ("My name is {0}, {1} {0}", "Bond", "James");
    #[allow(dead_code)]
    //struct Structure(i32);
    //impl fmt::Display */ 
    //print!("This struct `{}` won't print...", Structure(3));
    let numbers = format!("{1} {} {0} {}", 1, 3);
    println!("{}", numbers);
    let pi = 3.141592; 
    println! ("pi is roughly {:.6}", pi);
}

/* This is a simple Rust program */