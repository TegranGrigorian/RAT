use std::{fs::File, path};
use tar::Builder;
use std::io;

pub struct TarCompressManager{}

impl TarCompressManager {
    pub fn compress(input: &str, output: &str) -> io::Result<()> {
        let tar_gz = File::create(output)?;
        let mut builder = Builder::new(tar_gz);
        builder.append_path(input)?;
        Ok(())
    }
}