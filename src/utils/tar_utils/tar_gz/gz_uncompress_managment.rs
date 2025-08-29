
use std::fs::File;
use flate2::read::GzDecoder;
use tar::Archive;

pub struct TarUncompressManager {}

impl TarUncompressManager {
    pub fn decompress(&self, input_path: &str, output_path: &str) -> std::io::Result<()> {
        // Check if the input file exists
        if !std::path::Path::new(input_path).exists() {
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Input file does not exist"));
        }

        println!("Decompressing {} to {}", input_path, output_path);
        let tar_gz_file = File::open(input_path)?;
        let tar_gz_decoder = GzDecoder::new(tar_gz_file);
        let mut archive = Archive::new(tar_gz_decoder);
        archive.unpack(output_path)?;
        Ok(())
    }
}