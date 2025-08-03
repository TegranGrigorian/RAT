use std::fs::File;

pub struct FileUtil {}

impl FileUtil {
    pub fn create_file(path: &str) -> std::io::Result<File> {
        File::create(path)
    }
    
    pub fn delete_file(path: &str) -> std::io::Result<()> {
        std::fs::remove_file(path)
    }
}