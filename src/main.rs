pub mod manager;

use manager::{Operations, FileManager};

fn main() {
    let file_manager = FileManager;

    file_manager.read_file("./src/main.rs").expect("TODO: panic message");

    //file_manager.write_file();

    //file_manager.list_files("./");

    // match file_manager.create_file("file.txt") {
    //     Ok(()) => println!("File created successfully."),
    //     Err(e) => panic!("File creation failed: {:?}", e),
    // }
    //
    // match file_manager.create_directory("dir_name") {
    //     Ok(()) => println!("Directory created successfully."),
    //     Err(e) => panic!("Directory creation failed: {:?}", e),
    // }
}
