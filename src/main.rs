use std::io::{self, Write};

fn main() {
    loop{
        print!("kudOS> ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        println!("Command Received: {}", input.trim());
    }
    // println!("Hello, world!");
}
