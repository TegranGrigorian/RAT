use crate::utils::tar_utils::tar_compress_managment::TarCompressManager;

pub fn delete_file(path: &str) -> std::io::Result<()> { //helper for a delete flag in the workflow
    crate::utils::file_util::FileUtil::delete_file(path)
}

pub fn compress_folder(input_path: &str, output_path: &str) -> std::io::Result<()> {
    let manager = TarCompressManager {};
    manager.compress(input_path, &mut output_path.to_string())
}

pub fn decompress_folder(input_path: &str, output_path: &str) -> std::io::Result<()> {
    let manager = crate::utils::tar_utils::tar_uncompress_managment::TarUncompressManager {};
    manager.decompress(input_path, output_path)
}

