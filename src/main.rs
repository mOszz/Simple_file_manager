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

fn load_command() -> Vec<Command> {
    let commands: Vec<Command> = vec![
        //Single arg commands
        Command {
            name: "List".to_string(),
            command: "l".to_string(),
            args: (vec![String::from("<directory>")], false),
            description: "Command to list all directory or specific one".to_string(),
            action: Box::new(|fm, args| FileManager::list_files(fm, &args[0])),
        },
        Command {
            name: "Create File".to_string(),
            command: "file".to_string(),
            args: (vec![String::from("<file name>")], true),
            description: "Command to create a file".to_string(),
            action: Box::new(|fm, args| FileManager::create_file(fm, &args[0])),
        },
        Command {
            name: "Create Directory".to_string(),
            command: "dir".to_string(),
            args: (vec![String::from("<directory name>")], true),
            description: "Command to create a directory".to_string(),
            action: Box::new(|fm, args| FileManager::create_directory(fm, &args[0])),
        },
        Command {
            name: "Read File".to_string(),
            command: "read".to_string(),
            args: (vec![String::from("<file name>")], true),
            description: "Command to read a file".to_string(),
            action: Box::new(|fm, args| FileManager::read_file(fm, &args[0])),
        },
        Command {
            name: "Delete File".to_string(),
            command: "delf".to_string(),
            args: (vec![String::from("<file name>")], true),
            description: "Command to delete a file".to_string(),
            action: Box::new(|fm, args| FileManager::delete_file(fm, &args[0])),
        },
        Command {
            name: "Delete Directory".to_string(),
            command: "deld".to_string(),
            args: (vec![String::from("<directory name>")], true),
            description: "Command to delete a directory".to_string(),
            action: Box::new(|fm, args| FileManager::delete_directory(fm, &args[0])),
        },
        //Multiple args commands
        Command {
            name: "Write file".to_string(),
            command: "write".to_string(),
            args: (vec![String::from("<file name>"), String::from("<content>")], true),
            description: "Command to write in a file".to_string(),
            action: Box::new(|fm, args| FileManager::write_file(fm, &args[0], &args[1])),
        },
        Command {
            name: "Copy file".to_string(),
            command: "copy".to_string(),
            args: (vec![String::from("<file name>"), String::from("<destination>")], true),
            description: "Command to copy a file".to_string(),
            action: Box::new(|fm, args| FileManager::copy_file(fm, &args[0], &args[1])),
        },
        Command {
            name: "Rename or Move".to_string(),
            command: "move".to_string(),
            args: (vec![String::from("<file name>"), String::from("<new file name>")], true),
            description: "Command to rename or move a file".to_string(),
            action: Box::new(|fm, args| FileManager::rename_or_move_file(fm, &args[0], &args[1])),
        },
    ];
    commands
}

fn main() {
    let commands = load_command();

    loop {
        app_running();

        let user_input = Command::read_user_input();
        let parsed_input = Command::parse_user_input(&user_input);
        let (command, args) = Command::identify_args(parsed_input);

        Command::execute_command(&command, args, &commands);
    }
}


