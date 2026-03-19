pub mod react;
pub mod tauri;

use std::fs;

enum FileType {
    Dir(String),
    File(String),
}

impl FileType {
    fn dir(path: &str) -> Self {
        FileType::Dir(path.to_string())
    }

    fn file(path: &str) -> Self {
        FileType::File(path.to_string())
    }

    fn create(&self) {
        match self {
            FileType::Dir(path) => { fs::create_dir_all(path).unwrap(); }
            FileType::File(path) => { fs::File::create(path).unwrap(); }
        }
    }
}