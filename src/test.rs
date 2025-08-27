// contains tests of methods contained
use crate::utils::tar_utils::xz_compress_managment::XzCompressManagment;

pub fn test_xz_compress(file_path: &str) {
    let output_path = format!("{}.xz", file_path);
    XzCompressManagment::compress_file(file_path, &output_path, 6).unwrap();
}