// contains tests of methods contained
use crate::utils::tar_utils::tar::tar_compress_managment::TarCompressManager;
use crate::utils::tar_utils::tar_xz::xz_compress_managment::XzCompressManagment;
use crate::utils::tar_utils::tar_xz::xz_uncompress_managment::XzUncompressManagment;

// test functions
pub fn test_xz_compress(file_path: &str) {
    let output_path = format!("{}.xz", file_path);
    XzCompressManagment::compress_file(file_path, &output_path, 6).unwrap();
}

pub fn test_xz_uncompress(file_path: &str) {
    XzUncompressManagment::uncompress_file(file_path, "./").unwrap();
}

pub fn test_tar_compress(file_path: &str) {
    let output_path = format!("{}.tar", file_path);
    TarCompressManager::compress(file_path, &output_path).unwrap();
}