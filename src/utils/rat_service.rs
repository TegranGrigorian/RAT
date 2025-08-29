//holy imports
use std::path::Path;
use crate::utils::file_util::FileUtil;
use crate::utils::tar_utils::tar_gz::gz_uncompress_managment::TarUncompressManager;
use crate::utils::tar_utils::tar_xz::xz_compress_managment::XzCompressManagment;
use crate::utils::tar_utils::tar_xz::xz_uncompress_managment::XzUncompressManagment;
use crate::utils::tar_utils::tar_gz::gz_compress_managment::TarCompressManager;

fn workflow_manager(output_folder_name: &str) {
    let extracted_path = Path::new(output_folder_name);
    if extracted_path.exists() {
        println!("Extracted path exists: {:?}", extracted_path);
        let entries: Vec<_> = extracted_path.read_dir().expect("Failed to read directory").collect();
        println!("Entries found: {}", entries.len());
        if entries.len() == 1 {
            if let Ok(entry) = &entries[0] {
                let file_name = entry.file_name();
                println!("Processing top-level entry: {:?}", file_name);
                if file_name.to_str() == Some(output_folder_name) {
                    let inner_path = entry.path();
                    println!("Found inner folder: {:?}", inner_path);
                    fn move_contents_recursively(src: &std::path::Path, dest: &std::path::Path) -> Vec<(std::path::PathBuf, std::path::PathBuf)> {
                        let mut files_to_move_later = Vec::new();
                        for entry in src.read_dir().expect("Failed to read directory") {
                            if let Ok(entry) = entry {
                                let file_name = entry.file_name();
                                let new_path = dest.join(&file_name);
                                if entry.file_type().expect("Failed to get file type").is_dir() {
                                    println!("Recursively moving directory {:?} to {:?}", entry.path(), new_path);
                                    std::fs::create_dir_all(&new_path).expect("Failed to create directory");
                                    let nested_delayed = move_contents_recursively(&entry.path(), &new_path);
                                    files_to_move_later.extend(nested_delayed);
                                    std::fs::remove_dir(entry.path()).expect("Failed to remove directory");
                                } else {
                                    if new_path == src {
                                        let temp_name = format!(".temp_{}", file_name.to_string_lossy());
                                        let temp_path = dest.join(&temp_name);
                                        println!("Moving conflicting file {:?} to temporary location {:?}", entry.path(), temp_path);
                                        std::fs::rename(entry.path(), &temp_path).expect("Failed to move to temp location");
                                        files_to_move_later.push((temp_path, new_path));
                                        println!("File {:?} conflicts with source directory, moved to temp, will move to final location after cleanup", file_name);
                                    } else if new_path.exists() {
                                        println!("Conflict detected: {:?} already exists", new_path);
                                        let mut counter = 1;
                                        let mut renamed_path = dest.join(format!("{}_{}", file_name.to_string_lossy(), counter));
                                        while renamed_path.exists() {
                                            counter += 1;
                                            renamed_path = dest.join(format!("{}_{}", file_name.to_string_lossy(), counter));
                                        }
                                        println!("Renaming file {:?} to avoid conflict: {:?}", entry.path(), renamed_path);
                                        std::fs::rename(entry.path(), renamed_path).expect("Failed to rename file");
                                    } else {
                                        println!("Moving file {:?} to {:?}", entry.path(), new_path);
                                        std::fs::rename(entry.path(), &new_path).expect("Failed to move file");
                                    }
                                }
                            }
                        }
                        files_to_move_later
                    }
                    println!("Recursively moving contents of {:?} to {:?}", inner_path, extracted_path);
                    let delayed_files = move_contents_recursively(&inner_path, &extracted_path);
                    println!("Removing inner folder: {:?}", inner_path);
                    std::fs::remove_dir_all(inner_path).expect("Failed to remove inner folder");
                    for (src_file, dest_file) in delayed_files {
                        if dest_file.exists() {
                            println!("Final move conflict detected: {:?} already exists", dest_file);
                            let mut counter = 1;
                            let file_name = src_file.file_name().unwrap().to_string_lossy();
                            let mut renamed_path = extracted_path.join(format!("{}_{}", file_name, counter));
                            while renamed_path.exists() {
                                counter += 1;
                                renamed_path = extracted_path.join(format!("{}_{}", file_name, counter));
                            }
                            println!("Renaming final file {:?} to avoid conflict: {:?}", src_file, renamed_path);
                            std::fs::rename(src_file, renamed_path).expect("Failed to rename final file");
                        } else {
                            println!("Moving final file {:?} to {:?}", src_file, dest_file);
                            std::fs::rename(src_file, dest_file).expect("Failed to move final file");
                        }
                    }
                }
            }
        }
    }
}

pub fn decompress_tar(input_path: &str, delete_flag: &bool) {
    let output_folder_name = Path::new(input_path)
        .file_stem()
        .and_then(|n| n.to_str())
        .unwrap_or("output_folder")
        .trim_end_matches(".tar")
        .to_string();
    let manager = TarUncompressManager {};
    println!("Decompressing {} to {}", input_path, output_folder_name);
    let _tar = manager.decompress(input_path, &output_folder_name);
    if let Ok(_) = _tar {
        workflow_manager(&output_folder_name);
        if *delete_flag {
            println!("Deleting input file: {}", input_path);
            let _delete = FileUtil::delete_file(input_path);
            if _delete.is_err() {
                println!("Error deleting the input file: {}", _delete.unwrap_err());
            } else {
                println!("Input file deleted successfully.");
            }
        }
        match _tar {
            Ok(_) => println!("Decompression successful!"),
            Err(e) => println!("Error during decompression: {}", e)
        }
    }
}

pub fn decompress_xz(input_path: &str, delete_flag: &bool) {
    let output_folder_name = Path::new(input_path)
        .file_stem()
        .and_then(|n| n.to_str())
        .unwrap_or("output_folder")
        .trim_end_matches(".tar")
        .to_string();
    println!("Decompressing {} to {}", input_path, output_folder_name);
    let _xz = XzUncompressManagment::uncompress_file(input_path, &output_folder_name);
    if let Ok(_) = _xz {
        workflow_manager(&output_folder_name);
        if *delete_flag {
            println!("Deleting input file: {}", input_path);
            let _delete = FileUtil::delete_file(input_path);
            if _delete.is_err() {
                println!("Error deleting the input file: {}", _delete.unwrap_err());
            } else {
                println!("Input file deleted successfully.");
            }
        }
        match _xz {
            Ok(_) => println!("Decompression successful!"),
            Err(e) => println!("Error during decompression: {}", e)
        }
    }
}

pub fn compress_tar(input_path: &str, output_path: &str, delete_flag: &bool) {
    let manager = TarCompressManager {};
    println!("Compressing {} to {}", input_path, output_path);
    let _tar = manager.compress(input_path, &mut output_path.to_string());
    match _tar {
        Ok(_) => {
            println!("Compression successful!");
            if *delete_flag {
                println!("Deleting input file: {}", input_path);
                let _delete = FileUtil::delete_file(input_path);
                if _delete.is_err() {
                    println!("Error deleting the input file: {}", _delete.unwrap_err());
                } else {
                    println!("Input file deleted successfully.");
                }
            }
        },
        Err(e) => println!("Error during compression: {}", e)
    }
}

pub fn compress_xz(input_path: &str, output_path: &str, delete_flag: &bool) {
    let mut out_path = output_path.to_string();
    if !out_path.ends_with(".tar.xz") {
        out_path = format!("{}.tar.xz", out_path.trim_end_matches(".tar.gz").trim_end_matches(".gz"));
    }
    println!("Compressing {} to {}", input_path, out_path);
    let res = XzCompressManagment::compress_file(input_path, &out_path, 6);
    match res {
        Ok(_) => {
            println!("Compression successful!");
            if *delete_flag {
                println!("Deleting input file: {}", input_path);
                let _delete = FileUtil::delete_file(input_path);
                if _delete.is_err() {
                    println!("Error deleting the input file: {}", _delete.unwrap_err());
                } else {
                    println!("Input file deleted successfully.");
                }
            }
        },
        Err(e) => println!("Error during compression: {}", e)
    }
}

pub fn delete_file(path: &str) -> std::io::Result<()> { //helper for a delete flag in the workflow
    crate::utils::file_util::FileUtil::delete_file(path) // a little gross but meh
}

pub fn compress_folder(input_path: &str, output_path: &str) -> std::io::Result<()> {
    let manager = TarCompressManager {};
    manager.compress(input_path, &mut output_path.to_string())
}

pub fn decompress_folder(input_path: &str, output_path: &str) -> std::io::Result<()> {
    let manager = TarUncompressManager {};
    // let manager = crate::utils::tar_utils::tar_gz_uncompress_managment::TarUncompressManager {};
    manager.decompress(input_path, output_path)
}

pub fn compress_folder_xz(input_path: &str, output_path: &str) -> std::io::Result<()> {
    let manager = TarCompressManager {};
    manager.compress(input_path, &mut output_path.to_string())
}

pub fn decompress_folder_xz(input_path: &str, output_path: &str) -> std::io::Result<()> {
    let manager = TarUncompressManager {};
    manager.decompress(input_path, output_path)
}