use std::io;

fn main() {
    println!("Input a string");
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    println!("{}", find_first_word(&line));
}

fn find_first_word(str: &str) -> &str {
    let parts: Vec<_> = str.split(" ").collect();
    return parts[0];
}