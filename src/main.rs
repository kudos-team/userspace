use std::io::{self, Write};
use std::process::Command;

fn main() {
    loop{
        print!("kudOS> ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let args : Vec<&str> = input.split_whitespace().collect();

        println!("Args are: {:?}", args);

        execute_commands(args[0]);
    }
    // println!("Hello, world!");
}

fn execute_commands(cmd: &str){
    let mut cmd = Command::new(cmd);

    let child_proc = cmd.spawn().expect("Failed to spawn the command");

    let output = child_proc.wait_with_output().expect("Failed to wait for this command");

    let stdout_str = String::from_utf8_lossy(&output.stdout);

    print!("{}", stdout_str);
}