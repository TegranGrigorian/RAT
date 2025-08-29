use tar::Builder;
use std::fs::File;
use flate2::Compression;
use flate2::write::GzEncoder;

pub struct TarCompressManager {}

impl TarCompressManager {
    pub fn check_path(&self, path: &str) -> bool {
        // Check if the path exists and is a directory
        std::path::Path::new(path).is_dir()
    }
    pub fn compress(&self, input_path: &str, output_path: &mut String) -> std::io::Result<()> { // Input: input path, output: optional file name/path
        if output_path.is_empty() {
            *output_path = format!("{}.tar.gz", input_path);
        }
        if self.check_path(input_path) {
            println!("Compressing {} to {}", input_path, output_path);
        } else {
            println!("Input path does not exist or is not a directory: {}", input_path);
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Input path does not exist or is not a directory"));
        }
        let output_file = File::create(output_path)?;
        let encoder = GzEncoder::new(output_file, Compression::default());
        let mut tar_builder = Builder::new(encoder);
        tar_builder.append_dir_all("./", input_path)?;
        tar_builder.finish()?;
        Ok(())
    }
}