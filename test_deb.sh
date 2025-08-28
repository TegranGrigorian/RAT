!#/bin/bash

# this script test the xz normal gz compression

rat -x src/test_folder

#check if this file exists, test_folder.tar.xz
if [ -f "test_folder.tar.xz" ]; then
    echo "File test_folder.tar.xz exists."
else
    echo "File test_folder.tar.xz does not exist."
    exit 1
fi

rat test_folder.tar.xz #now try and uncomrpess it

if [ -d "test_folder" ]; then
    echo "Directory test_folder exists."
else
    echo "Directory test_folder does not exist."
    exit 1
fi

# --- This is for gz now ---

rat src/test_folder

#check if this file exists, test_folder.tar.gz
if [ -f "test_folder.tar.gz" ]; then
    echo "File test_folder.tar.gz exists."
else
    echo "File test_folder.tar.gz does not exist."
    echo "Directory test_folder does not exist."
    exit 1
fi

rat test_folder.tar.gz #now try and uncomrpess it

if [ -d "test_folder" ]; then
    echo "Directory test_folder exists."
else
    echo "Directory test_folder does not exist."
    exit 1
fi

# delete all files
rm -rf test_folder.tar.gz test_folder test_folder.tar.xz