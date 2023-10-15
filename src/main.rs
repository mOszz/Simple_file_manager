pub mod manager;

use manager::{Operations, FileManager};

fn main() {
    let file_manager = FileManager::new();
    file_manager.list_files("src").expect("TODO: panic message");
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
