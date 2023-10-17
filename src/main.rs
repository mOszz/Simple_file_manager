pub mod manager;

use std::io;
use std::io::{stdout, Write};
use manager::{Operations, FileManager};

fn app_running() {
    let prompt_char = '🦀';

    print!("{} ", prompt_char);
    stdout().flush().unwrap();
}

fn read_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read in command");

    return input.trim().to_string();
}

fn parse_user_input(input: &str) -> Vec<String> {
    input.trim().split_whitespace().map(String::from).collect()
}

fn identify_args(words: Vec<String>) -> (String, Vec<String>) {
    if words.is_empty() {
        return (String::new(), Vec::new());
    }
    let command = words[0].clone();
    let args = words[1..].to_vec();
    (command, args)
}

fn main() {
    loop {
        app_running();
        let file_manager = FileManager::new();

        let user_input = read_user_input();
        let parsed_input = parse_user_input(&user_input);
        let (command, mut args) = identify_args(parsed_input);
        match command.as_str() {
            "list" => {
                if args[0].is_empty() {
                    args[0] = String::from("./");
                }
                println!("{}", args[0]);
                //file_manager.list_files(args[0].as_str());
            },
            _ => {
                println!("Command didn't exist");
                //Ok(());
            }
        };
    }

    //file_manager.list_files("src").expect("TODO: panic message");
    //file_manager.create_file("test.txt");
    //file_manager.create_file("C:/Users/victo/Documents/test.txt");
    //file_manager.read_file("C:/Users/victo/Documents/dev/rust/simple_file_manager/src/main.rs");
    //file_manager.write_file("C:/Users/victo/Documents/dev/rust/simple_file_manager/test.txt", "udidhbdi");
    //file_manager.copy_file("test.txt", "src/test.txt");
    //file_manager.rename_or_move_file("dddd", "dddd");
    //file_manager.delete_file("sdf");
    //file_manager.create_directory("./src");
    //file_manager.list_files("/toto/failed test");
    //file_manager.delete_file("./test/delete/folder");
}
