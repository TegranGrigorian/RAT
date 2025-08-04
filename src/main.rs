use rat::utils::tar_utils::{tar_compress_managment::TarCompressManager, tar_uncompress_managment::TarUncompressManager};
use rat::utils::file_util::FileUtil;
use std::env::args;
use std::path::Path;
#[allow(non_snake_case)]
fn main() {
    // let mut args = args();
    // let input_path = args.nth(1).expect("Input path not provided");
    // let output_path = args.next();
    // let output_path = match output_path {
    //     Some(path) => format!("{}.tar.gz", path),
    //     None => format!("{}.tar.gz", input_path),
    // };    
    let args: Vec<String> = args().skip(1).collect();    
    let mut input_path = None;
    let mut output_path: Option<String> = None;
    let mut delete_flag = false;

    for arg in &args {
        if arg == "-d" {
            delete_flag = true;
        } else if arg == "--help" || arg == "-h" {
            println!("Rat - Rust Archive Tool or tar in reverse");
            println!("A simple tool to compress and decompress files and folders using tar.gz format.");
            println!("Usage: rat [options] <input_path> [output_path]");
            println!("Arguments:");
            println!("  <input_path>    The path to the input file or folder to compress/decompress");
            println!("  [output_path]   The path to the output file. If not provided, it will be set to <input_path>.tar.gz");
            println!("output path not needed for decompression, it will be set to output_folder");
            println!("Options:");
            println!("  -d, --delete    Delete the input file after compression/decompression");
            println!("  -h, --help      Show this help message");
            println!("Examples:");
            println!("  rat input_folder");
            println!("  rat input_folder output_file.tar.gz");
            println!("  rat -d input_folder output_file.tar.gz");
            println!("  rat -d input_file.tar.gz");
            return;
        } else if arg.ends_with(".tar.gz") {
            // If the argument ends with .tar.gz, assign it to input_path if not already set
            if input_path.is_none() {
                input_path = Some(arg.clone());
            } else if output_path.is_none() {
                // Otherwise, assign it to output_path if input_path is already set
                output_path = Some(arg.clone());
            }
        } else if input_path.is_none() {
            // Assign the first non-.tar.gz argument to input_path
            input_path = Some(arg.clone());
        } else if output_path.is_none() {
            // Assign the next argument to output_path, appending .tar.gz if necessary
            output_path = Some(format!("{}.tar.gz", arg));
        }
    }
    // let input_path = "test_folder";
    // let output_path = "new_output.tar.gz";

    // add logic for figuring out if a user targets a folder or a file
    // if its a folder, compress it, file then uncompress it
    let input_path = input_path.expect("Input path not provided");
    let folder_name = Path::new(&input_path)
    .file_name()
    .and_then(|n| n.to_str())
    .unwrap_or(&input_path);

    let output_path = output_path.unwrap_or_else(|| format!("{}.tar.gz", folder_name));
    let output_folder_name = if input_path.ends_with(".tar.gz") {
        // Extract the base name of the .tar.gz file to use as the output folder name
        Path::new(&input_path)
            .file_stem()
            .and_then(|n| n.to_str())
            .unwrap_or("output_folder")
            .trim_end_matches(".tar") // Remove .tar if present
            .to_string()
    } else {
        "output_folder".to_string()
    };

    if input_path.ends_with(".tar.gz") {
        let manager = TarUncompressManager {};
        println!("Decompressing {} to {}", input_path, output_folder_name);
        let _tar = manager.decompress(input_path.as_str(), &output_folder_name);
        if let Ok(_) = _tar {
            let extracted_path = Path::new(&output_folder_name);
            if extracted_path.exists() {
                println!("Extracted path exists: {:?}", extracted_path);
                let entries: Vec<_> = extracted_path.read_dir().expect("Failed to read directory").collect();
                println!("Entries found: {}", entries.len());
                if entries.len() == 1 {
                    if let Ok(entry) = &entries[0] {
                        let file_name = entry.file_name();
                        println!("Processing top-level entry: {:?}", file_name);
                        if file_name.to_str() == Some(output_folder_name.as_str()) {
                            let inner_path = entry.path();
                            println!("Found inner folder: {:?}", inner_path);
                            // Move folder deletion logic before processing inner entries
                            // Process inner entries before removing the folder
                            // First, recursively move contents of directories to the parent directory
                            fn move_contents_recursively(src: &std::path::Path, dest: &std::path::Path) -> Vec<(std::path::PathBuf, std::path::PathBuf)> {
                                let mut files_to_move_later = Vec::new();
                                
                                // First pass: process directories and collect files that conflict with source path
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
                                            // Check if this file would conflict with the source directory name
                                            if new_path == src {
                                                // This file has the same name as the source directory, move it to temp location first
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

                            // Remove the folder after moving its contents
                            println!("Removing inner folder: {:?}", inner_path);
                            std::fs::remove_dir_all(inner_path).expect("Failed to remove inner folder");

                            // Now move the delayed files after directory removal
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
            if delete_flag {
                println!("Deleting input file: {}", input_path);
                let _delete = FileUtil::delete_file(&input_path);
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
    } else {
        let manager = TarCompressManager {};
        println!("Compressing {} to {}", input_path, output_path);
        let _tar = manager.compress(input_path.as_str(), &mut output_path.to_string());
    }
    

    // let input_path = "new_output.tar.gz";
    // let manager = TarUncompressManager {};
    // let tar = manager.decompress(input_path, "output_folder");
    // match tar {
    //     Ok(_) => println!("Decompression successful!"),
    //     Err(e) => println!("Error during decompression: {}", e)
    // }
}
