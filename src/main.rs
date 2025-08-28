use rat::utils::rat_service;
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
    
    //NOTE: Test Statments
    // test::test_xz_compress("/home/tegran-grigorian/Documents/Projects/RAT/src/test_folder");
    // test::test_xz_uncompress("/home/tegran-grigorian/Documents/Projects/RAT/test_folder.xz"); //this doenst exist yet :)
    
    // Main workflow
    let args: Vec<String> = args().skip(1).collect();
    let mut input_path = None;
    let mut output_path: Option<String> = None;
    let mut delete_flag = false;
    let mut xz_flag = false;

    for arg in &args {
        if arg == "-d" {
            delete_flag = true;
        } else if arg == "-x" || arg == "--xz" {
            xz_flag = true;
        } else if arg == "--help" || arg == "-h" {
            println!("Rat - Rust Archive Tool or tar in reverse");
            println!("A simple tool to compress and decompress files and folders using tar.gz format.");
            println!("Usage: rat [options] <input_path> [output_path]");
            println!("Arguments:");
            println!("  <input_path>    The path to the input file or folder to compress/decompress");
            println!("  [output_path]   The path to the output file. If not provided, it will be set to <input_path>.tar.gz");
            println!("output path not needed for decompression, it will be set to output_folder");
            println!("Options:");
            println!("  -x, --xz          Use xz compression");
            println!("  -d, --delete      Delete the input file after compression/decompression");
            println!("  -h, --help        Show this help message");
            println!("Examples:");
            println!("  rat input_folder");
            println!("  rat input_folder output_file.tar.gz");
            println!("  rat -d input_folder output_file.tar.gz");
            println!("  rat -d input_file.tar.gz");
            return;
        } else if arg.starts_with('-') {
            // skip any other flags
            continue;
        } else if arg.ends_with(".tar.gz") {
            if input_path.is_none() {
                input_path = Some(arg.clone());
            } else if output_path.is_none() {
                output_path = Some(arg.clone());
            }
        } else if input_path.is_none() {
            input_path = Some(arg.clone());
        } else if output_path.is_none() {
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
    // Dispatch to rat_service for all workflows
    if input_path.ends_with(".tar.xz") {
        rat_service::decompress_xz(&input_path, &delete_flag);
    } else if input_path.ends_with(".tar.gz") {
        rat_service::decompress_tar(&input_path, &delete_flag);
    } else if xz_flag {
        rat_service::compress_xz(&input_path, &output_path, &delete_flag);
    } else {
        rat_service::compress_tar(&input_path, &output_path, &delete_flag);
    }
    

    // let input_path = "new_output.tar.gz";
    // let manager = TarUncompressManager {};
    // let tar = manager.decompress(input_path, "output_folder");
    // match tar {
    //     Ok(_) => println!("Decompression successful!"),
    //     Err(e) => println!("Error during decompression: {}", e)
    // }

}
