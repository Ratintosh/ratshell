mod cmds;

use std::io::{stdin, stdout, Write};

fn input() -> String {   
    print!("\x1b[0;92;49m->\x1b[0m ");
    stdout().flush().unwrap();

    let mut text: String = String::new();
    stdin()
        .read_line(&mut text)
        .expect("Error...");
    return text;
}

fn prompt() {   
    loop {
        let raw_input: String = input();
        let input: Vec<&str> = raw_input.split_whitespace().collect();
        if input.is_empty() {
            continue;
        }
        let cmd: &str = input[0];
        let args: Vec<&str> = input[1..].to_vec();
        
        

        match cmd {
            "help" => cmds::help::main(args),
            "foo" => cmds::foobar::main(),
            "cowsay" => cmds::cowsay::main(args),
            "echo" => cmds::echo::main(args),
            "exit" => break,
            _ => println!("Unknown command: {}", cmd),
        }
    }
}

fn main() {
    print!("\x1B[2J");
    println!("Welcome to ratshell.\nType 'help' for a list of commands.\n");
    //println!("\x1b[93mError\x1b[0m");
    prompt();
}