mod cmds;

use std::io::{stdin, stdout, Write};
use color_print::{cprintln, cprint};
use chrono::Local;

fn input() -> String {   
    cprint!("<rgb(125, 169, 254)>-> </>");
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
            "sort" => cmds::sort::main(args),
            "exit" => break,
            _ => println!("Unknown command: {}", cmd),
        }
    }
}

fn main() {
    print!("\x1B[2J");
    cprintln!(r#"
<rgb(10, 108, 255)>:::::::..    :::. :::::::::::: .::::::.   ::   .: .,::::::   :::      :::     </>
<rgb(39, 123, 255)>;;;;``;;;;   ;;`;;;;;;;;;;'''';;;`    `  ,;;   ;;,;;;;''''   ;;;      ;;;     </>
<rgb(68, 138, 255)> [[[,/[[['  ,[[ '[[,   [[     '[==/[[[[,,[[[,,,[[[ [[cccc    [[[      [[[     </>
<rgb(96, 154, 254)> $$$$$$c   c$$$cc$$$c  $$       '''    $"$$$"""$$$ $$""""    $$'      $$'     </>
<rgb(125, 169, 254)> 888b "88bo,888   888, 88,     88b    dP 888   "88o888oo,__ o88oo,.__o88oo,.__</>
<rgb(154, 184, 254)> MMMM   "W" YMM   ""`  MMM      "YMmMY"  MMM    YMM""""YUMMM""""YUMMM""""YUMMM</>
    "#);
    let now = Local::now();

    cprintln!("<rgb(109, 109, 109)>// Logged in at {} //</>\n", now.format("%Y-%m-%d %H:%M:%S"));
    cprintln!("<rgb(125, 169, 254)>[SYS] </>Welcome to ratshell.");
    cprintln!("<rgb(125, 169, 254)>[SYS] </>Type 'help' for a list of commands.\n");
    //println!("\x1b[93mError\x1b[0m");
    prompt();
}