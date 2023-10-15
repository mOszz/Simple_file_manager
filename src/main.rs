pub mod manager;

use manager::{Operations, FileManager};

fn main() {
    let file_manager = FileManager::new();
    file_manager.list_files("src").expect("TODO: panic message");
    //file_manager.create_file("jest");
    //file_manager.read_file("./hubs.txt");
    //file_manager.write_file("zoo.xps", "udidhbdi");
    //file_manager.copy_file("gevent", "dddd");
    //file_manager.rename_or_move_file("dddd", "dddd");
    //file_manager.delete_file("sdf");
    //file_manager.create_directory("./src");
    //file_manager.list_files("/toto/failed test");
    //file_manager.delete_file("./test/delete/folder");
}
