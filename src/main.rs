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
        } else if arg.ends_with(".tar.gz") && input_path.is_none() {
            input_path = Some(arg.clone());
        } else if input_path.is_none() {
            input_path = Some(arg.clone());
        } else if output_path.is_none() {
            if arg.ends_with(".tar.gz") {
                output_path = Some(arg.clone());
            } else {
                output_path = Some(format!("{}.tar.gz", arg));
            }
            // match output_path {
            //     Some(ref path) if path.ends_with(".tar.gz") => {
            //         output_path = Some(format!("{}.tar.gz", path));
            //     }
            //     _ => {
            //         output_path = Some(format!("{}.tar.gz", input_path.as_ref().unwrap()));
            //     }
            // }
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
    if input_path.ends_with(".tar.gz") {
        let manager = TarUncompressManager {};
        let _tar = manager.decompress(input_path.as_str(), "output_folder");
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
    