pub mod manager;

use manager::{Operations, FileManager};

fn main() {
    let file_manager = FileManager::new();

    file_manager.list_files("").expect("TODO: panic message");

    //file_manager.write_file();


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
