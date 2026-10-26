use std::io;

fn main() {
    println!("Please enter a text to count its words");

    let mut text = String::new();

    io::stdin()
        .read_line(&mut text)
        .expect("Failed to read line");

    let mut words: Vec<&str> = text.trim().split_whitespace().collect();
 
    let word_count = words.len(); 

    println!("The text is {} words long", word_count);
 
    words.sort();
    words.dedup();
    //let unique_words: HashSet<&str> = words.iter().copied().collect();

    println!("Words found: {:?}", words); 
    println!("Helios");
}
