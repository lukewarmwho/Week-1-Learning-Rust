/*fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds is: {THREE_HOURS_IN_SECONDS}");
}*/
/*fn main() {
    let x = 5; //can also use "let mut x = 5;" to make it mutable

    let x = x + 1; // can also use "x = x + 1;" if "let mut x = 5;" is used instead of "let x = 5;"

    { 
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   "; //here, let mu spaces = "   "; would not work, as we're not allowed to mutate a variable's type.
    let spaces = spaces.len(); 
    println!("The number of spaces is: {spaces}"); 

    let _a1: f32 = 2.0;

    let _ay: f32 = 3.0;


} */

/*fn main() {

    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;

    let remainder = 43 % 5;

    println!("{sum}, {difference}, {product}, {quotient}, {truncated}, {remainder}");
    
    let t = true;

    let f = false;
    println!("t: {t}, f: {f}");

    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("c: {c}, z: {z}, heart_eyed_cat: {heart_eyed_cat}");

    let tup: (i32, f64, i8) = (-500, 6.4, -4);
    let (x, y, z,) = tup;
    println! ("z: {z}, y: {y}, x: {x}");
    println! ("tup.0: {}, tup.1: {}, tup.2: {}", tup.0, tup.1, tup.2);
    println! ("tup: {:#?}", tup);

    let y: (i32, f64, i8, u8) = (-500, 6.4, -4, 123);
    let a = y.0;
    let b = y.1;
    let c = y.2;
    let d = y.3;
    println!("{a}, {b}, {c}, {d}");

    let p = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let best_month  = p[1];
    println!("{best_month} is the best month!");

    let ab: [i32; 5] = [1, 2, 3, 4, 5];
    let bc = [3; 5];
    println! ("{:?}", bc);

} */

use std::io;
fn main () {
    let a = [1, 2, 3, 4, 5];

    loop {
        println!("Please enter an array index.");

        let mut index = String::new();

        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line"); 

        let index: usize = match index.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let element = a[index];

        println!("The value of the element at index {index} is: {element}");
    }
}