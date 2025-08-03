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
        let _tar = manager.decompress(input_path.as_str(), &output_folder_name);
        if let Ok(_) = _tar {
            let extracted_path = Path::new(&output_folder_name);
            if extracted_path.exists() {
                let entries: Vec<_> = extracted_path.read_dir().expect("Failed to read directory").collect();
                println!("Entries found: {}", entries.len());
                if entries.len() == 1 {
                    if let Ok(entry) = &entries[0] {
                        let file_name = entry.file_name();
                        if file_name.to_str() == Some(output_folder_name.as_str()) {
                            let inner_path = entry.path();
                            for inner_entry in inner_path.read_dir().expect("Failed to read inner directory") {
                                if let Ok(inner_entry) = inner_entry {
                                    let new_path = extracted_path.join(inner_entry.file_name());
                                    if new_path.exists() {
                                        // Rename conflicting files instead of skipping
                                        let mut counter = 1;
                                        let mut renamed_path = new_path.clone();
                                        while renamed_path.exists() {
                                            renamed_path = extracted_path.join(format!("{}_{}", inner_entry.file_name().to_string_lossy(), counter));
                                            counter += 1;
                                        }
                                        println!("Conflict detected: {:?} already exists. Renaming to {:?}.", new_path, renamed_path);
                                        std::fs::rename(inner_entry.path(), renamed_path).expect("Failed to rename file");
                                    } else {
                                        std::fs::rename(inner_entry.path(), new_path).expect("Failed to move file");
                                    }
                                }
                            }
                            std::fs::remove_dir_all(inner_path).expect("Failed to remove inner folder");
                        }
                    }
                }

            }
            // Check if the extracted content already contains a top-level folder matching the output folder name
            // let extracted_path = Path::new(&output_folder_name);
            // if extracted_path.exists() {
            //     let entries: Vec<_> = extracted_path.read_dir().expect("Failed to read directory").collect();
            //     if entries.len() == 1 {
            //         if let Ok(entry) = &entries[0] {
            //             let file_name = entry.file_name();
            //             if file_name.to_str() == Some(output_folder_name.as_str()) {
            //                 // Move the contents of the inner folder to the parent directory
            //                 let inner_path = entry.path();
            //                 for inner_entry in inner_path.read_dir().expect("Failed to read inner directory") {
            //                     if let Ok(inner_entry) = inner_entry {
            //                         let new_path = extracted_path.join(inner_entry.file_name());
            //                         if new_path.exists() {
            //                             // Handle conflict: skip the file or rename it
            //                             println!("Conflict detected: {:?} already exists. Skipping.", new_path);
            //                         } else {
            //                             if inner_entry.path().is_dir() {
            //                                 // Recursively move subdirectories
            //                                 std::fs::rename(inner_entry.path(), new_path).expect("Failed to move directory");
            //                             } else {
            //                                 std::fs::rename(inner_entry.path(), new_path).expect("Failed to move file");
            //                             }
            //                         }
            //                     }
            //                 }
            //                 // Remove the now-empty inner folder
            //                 std::fs::remove_dir_all(inner_path).expect("Failed to remove inner folder");
            //             }
            //         }
            //     }
            // }
        }
        if delete_flag {
            //delete the input file
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
    } else {
        let manager = TarCompressManager {};
        
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
