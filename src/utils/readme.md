# File Structure

```
utils/
├── file_util.rs
├── mod.rs
├── rat_service.rs
├── readme.md
└── tar_utils
    ├── mod.rs
    ├── tar_compress_managment.rs
    └── tar_uncompress_managment.rs
```

## file_util.rs

Utility that allows for the managment of files. This is mainly used to delete files

## mod.rs

module imports

## rat_service.rs

Acts as the service/mid-layer between the lower level tar_utils and the higher level main workflow

## tar_utils

Collection of utilits to manage tar methods