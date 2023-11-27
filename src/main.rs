pub mod manager;
pub mod command;

use std::env::current_dir;
use std::io::{stdout, Write};
use manager::{Operations, FileManager};
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
    let fm = FileManager::new();

    loop {
        app_running();

        let user_input = Command::read_user_input();
        let parsed_input = Command::parse_user_input(&user_input);
        let (command, args) = Command::identify_args(parsed_input);

        let command_enum = match command.as_str() {
            "ls" => Command::List(args[0].clone()),
            "file" => Command::CreateFile(args[0].clone()),
            "mkdir" => Command::CreateDirectory(args[0].clone()),
            "read" => Command::ReadFile(args[0].clone()),
            "rm" => Command::DeleteFile(args[0].clone()),
            "rm-r" => Command::DeleteDirectory(args[0].clone()),
            "cd" => Command::ChangeDirectory(args[0].clone()),
            "wr" => Command::WriteFile(args[0].clone(), args[1].clone()),
            "cp" => Command::CopyFile(args[0].clone(), args[1].clone()),
            "mv" => Command::RenameOrMoveFile(args[0].clone(), args[1].clone()),
            "exit" => Command::Exit,
            "help" => Command::Help,
            _ => { println!("Command not found");
                continue;
            }
        };

        if let Err(err) = Command::execute_command(command_enum, &fm) {
            eprintln!("Command failed: {}", err);
        }
    }
}

