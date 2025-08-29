// use xz crate and tar archive
use lzma_tarball::writer::LZMATarballWriter;
use std::fs::File;
use std::path::{self, Path, PathBuf};
pub struct XzCompressManagment;

impl XzCompressManagment {
    pub fn compress_file(input_path: &str, output_path: &str, compression_level: u8) -> Result<(), Box<dyn std::error::Error>>  {
        let input_path = Path::new(input_path);
        let output_path = Path::new(output_path);

        // Create a new LZMA tarball writer with the specified compression level
        let tarball_writer = LZMATarballWriter::new()
            .set_compression_level(compression_level)
            .with_path(input_path, "./") // Use empty path to avoid nesting
            .unwrap()
            .set_output(output_path)
            .compress(|progress| {
			// The percentage is between 0.0 and 1.0
			// Multiply by 100 to get a percentage
			let percentage = progress.percentage * 100f32;

			// The number of bytes processed
			let processed = progress.bytes_processed;

			// The number of bytes processed per second
			let bps = progress.bytes_per_second;

			// Convert bytes per second to megabytes per second
			let mbps = (bps as f32) / 1024f32 / 1024f32;

			print!("\x1b[1A"); // Move cursor up
			println!("Progress: {:.2}% - Processed: {}B - Speed: {:.2}Mb/s", percentage, processed, mbps);
		}).unwrap();
        // Add the input file or directory to the tarball

        Ok(())
    }
}