use lzma_tarball::reader::LZMATarballReader;
use lzma_tarball::writer::LZMATarballWriter;
use std::fs::File;
use std::path::{self, Path, PathBuf};

pub struct XzUncompressManagment{}

impl XzUncompressManagment {
    pub fn uncompress_file(input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>>  {
        let input_path = Path::new(input_path);
        let output_path = Path::new(output_path);

        // Create a new LZMA tarball writer with the specified compression level
        let tarball_writer = LZMATarballReader::new()
            .set_archive(input_path).unwrap()
            .set_output_directory(output_path).unwrap()
            .decompress().unwrap();

        Ok(())
    }
}