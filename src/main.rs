pub mod manager;
pub mod command;
use std::io::{stdout, Write};
use manager::{Operations, FileManager};
use command::Command;

fn app_running() {
    let prompt_char = '🦀';
    print!("{} ", prompt_char);
    stdout().flush().unwrap();
}

fn main() {
    let commands: Vec<Command> = vec![
        //Single arg commands
        Command {
            name: "List".to_string(),
            command: "l".to_string(),
            args: (vec![String::from("<directory>")], false),
            description: "Command to list all directory or specific one".to_string(),
            action: FileManager::list_files,
        },
        Command {
            name: "Create File".to_string(),
            command: "file".to_string(),
            args: (vec![String::from("<file name>")], true),
            description: "Command to create a file".to_string(),
            action: FileManager::create_file,
        },
        Command {
            name: "Create Directory".to_string(),
            command: "dir".to_string(),
            args: (vec![String::from("<directory name>")], true),
            description: "Command to create a directory".to_string(),
            action: FileManager::create_directory,
        },
        Command {
            name: "Read File".to_string(),
            command: "read".to_string(),
            args: (vec![String::from("<file name>")], true),
            description: "Command to read a file".to_string(),
            action: FileManager::read_file,
        },
        Command {
            name: "Delete File".to_string(),
            command: "delf".to_string(),
            args: (vec![String::from("<file name>")], true),
            description: "Command to delete a file".to_string(),
            action: FileManager::delete_file,
        },
        Command {
            name: "Delete Directory".to_string(),
            command: "deld".to_string(),
            args: (vec![String::from("<directory name>")], true),
            description: "Command to delete a directory".to_string(),
            action: FileManager::delete_directory,
        },
    ];

    loop {
        app_running();

        let user_input = Command::read_user_input();
        let parsed_input = Command::parse_user_input(&user_input);
        let (command, args) = Command::identify_args(parsed_input);

        Command::execute_command(&command, args, &commands);
    }
}

    //file_manager.write_file("C:/Users/victor/Documents/dev/rust/simple_file_manager/test.txt", "udidhbdi");
    //file_manager.copy_file("test.txt", "src/test.txt");
    //file_manager.rename_or_move_file("dddd", "dddd");
    //file_manager.delete_file("sdf");
    //file_manager.create_directory("./src");
    //file_manager.list_files("/toto/failed test");
    //file_manager.delete_file("./test/delete/folder");

