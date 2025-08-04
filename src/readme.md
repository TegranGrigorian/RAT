# File structure

```
src/
├── lib.rs
├── main.rs
├── readme.md
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

# main.rs
Contains the main workflow as well as folder-executable managemnt code

# lib.rs
Imports modules form the utils folder

# utils
Folder containing utilites that aid in the exectuion of the main workflow