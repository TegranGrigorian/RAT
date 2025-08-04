# RAT (Rust Archive Tool) - tar in reverse

RAT is a simple and efficient command-line tool written in Rust for compressing and decompressing files and folders using the `.tar.gz` format. It is designed to be a user-friendly alternative to traditional tar utilities, with a focus on ease of use and automation.

## Features
- Compress folders or files into `.tar.gz` archives
- Decompress `.tar.gz` archives into folders
- Automatically handles duplicate/recursive folder structures
- Optionally delete input files after operation
- Simple, clear CLI interface

## Usage
```
Rat - Rust Archive Tool or tar in reverse
A simple tool to compress and decompress files and folders using tar.gz format.
Usage: rat [options] <input_path> [output_path]
Arguments:
  <input_path>    The path to the input file or folder to compress/decompress
  [output_path]   The path to the output file. If not provided, it will be set to <input_path>.tar.gz
output path not needed for decompression, it will be set to output_folder
Options:
  -d, --delete    Delete the input file after compression/decompression
  -h, --help      Show this help message
Examples:
  rat input_folder
  rat input_folder output_file.tar.gz
  rat -d input_folder output_file.tar.gz
  rat -d input_file.tar.gz
```

## Project Structure
```
rat
├── Cargo.lock
├── Cargo.toml
├── deb_setup.sh
├── docs
│   ├── commands_examples.md
│   ├── dev_docs
│   │   └── deb_create.md
│   └── directory_structure.md
├── install
│   ├── dev_tools
│   │   └── setup_install.sh
│   ├── linux
│   │   └── rat
│   │       ├── install.sh
│   │       └── rat
│   └── packages
├── readme.md
└── src
    ├── lib.rs
    ├── main.rs
    ├── test_folder
    │   ├── bar.txt
    │   └── foo.txt
    └── utils
        ├── file_util.rs
        ├── mod.rs
        ├── rat_service.rs
        └── tar_utils
            ├── mod.rs
            ├── tar_compress_managment.rs
            └── tar_uncompress_managment.rs
```

## Building and Installing

1. **Build the project:**
   ```sh
   cargo build --release
   ```
2. **(Optional) Use the provided script for packaging and installation:**
   ```sh
   ./deb_setup.sh
   ```

## License
bruh

---

For more details, see the `docs/` directory.