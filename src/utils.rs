use std::io::{self, Write};

pub fn user_input(text: String) -> String {
    println!("___________________________________");
    let mut input_buffer = String::new();
    print!("Enter {}: ", text);
    let _ = io::stdout().flush(); // glues the previous print! statements to the stdin
    let _bytes_read = io::stdin().read_line(&mut input_buffer).unwrap();
    // println!("***********************************");
    // println!("Todo: {}", line.trim());
    // println!("No. of bytes read: {}", bytes_read);
    println!("___________________________________");
    input_buffer
}

pub fn print_help() {
    println!();
    println!("Usage: bin [ACTION]");
    println!();
    println!("ACTIONS:");
    println!("  help        Show help menu");
    println!("  add         Add todos");
    println!("  show        Show todos");
    println!("  complete    Complete a todo");
    println!("  delete      Delete a todo");
    println!();
}
