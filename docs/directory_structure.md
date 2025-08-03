
# Simple Directory Structure
```
src/
    main.rs
        Contains the top layer of the workflow that gathers the user's input and sends that data to a parser
    utils/
        parser_util.rs
            Splits the data from the user to be read by program for inputs
        rat_service.rs
            This service takes the inputs from the parser_util.rs and feeds it to the proper workflows and tar utils.rs
        tar_utils/
            tar_compress_managment.rs
                Contains the backend utility that can compress folders
            tar_uncompress_managment.rs
                Contains the backend utility that can uncompress tar.gz files
```