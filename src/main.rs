// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 The KudOS team

use std::io::{self, Write};
use std::process::Command;

fn main() {
    loop{
        print!("kudOS> ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let args : Vec<&str> = input.split_whitespace().collect();

        // println!("Args are: {:?}", args);

        execute_commands(args);
    }
    // println!("Hello, world!");
}

fn execute_commands(args: Vec<&str>){
    if args.is_empty() { return; }

    let mut cmd = Command::new(args[0]);
    cmd.args(&args[1..]);

    // let child_proc = cmd.spawn();
    match cmd.spawn() {
        Ok(child_proc) => {

            match child_proc.wait_with_output(){
                Ok(output) => {
                    let stdout_str = String::from_utf8_lossy(&output.stdout);
                    print!("{}", stdout_str);
                }

                Err(e) => {
                    eprintln!("kudOS: error waiting for command: {}", e);
                }
            }

        }
        Err(_) => {
            eprint!("kudOS: command not found: {}\n", args[0]);
        }
    }

}