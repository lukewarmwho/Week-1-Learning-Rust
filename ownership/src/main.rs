/*fn main() {
    /*let mut s = String::from("hello");
    s.push_str(", world");

    println!("{s} {s}");

    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s2}, world!");

    let mut a = String::from("hello");
    a = String::from("ahoy");

    println!("{a}, world!");

    let a1 = String::from("hi");
    let a2 = a1.clone();

    println!("a1 = {a1}, a2 = {a2}");

    let b1 = 'w';
    let b2 = b1;

    println!("{b1}, {b2}");*/

    let s = String::from("hello");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);
}

fn takes_ownership(new_string: String) {
    println!("{new_string}");

}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}")
}
*/
 
/*fn main() {
    let s1 = give_owner();

    let s2 = String::from("Hello");

    let s3 = takes_and_gives_back(s2);

    println!("{}, {}", s3, s1);

}

fn give_owner() -> String {
    let some_string = String::from("world");

    some_string

}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
} */

use std::io; 

fn main() {

    println!("Please input a word");

    let mut word = String::new();

    io::stdin()
        .read_line(&mut word)
        .expect("Failed to read line");

    let word = word.trim().to_string();

    let (s2, len) = calculate_length(word);

    println!("The length of {s2} is {len}");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}