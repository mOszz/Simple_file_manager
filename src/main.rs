pub mod manager;
pub mod command;

use std::env::current_dir;
use std::io::{stdout, Write};
use command::Command;

fn app_running() {
    let prompt_char = '🦀';

    if let Ok(current_dir) = current_dir() {
        print!("{} {}>", prompt_char, current_dir.to_string_lossy());
    } else {
        eprintln!("Failed to get current directory");
    }
    stdout().flush().unwrap();
}

fn main() {
    loop {
        app_running();

        let user_input = Command::read_user_input();
        let parsed_input = Command::parse_user_input(&user_input);
        let (command, args) = Command::identify_args(parsed_input);

        if let Some(cmd) = Command::load_command(&command) {
            Command::execute_command(args, cmd);
        }
    }
}