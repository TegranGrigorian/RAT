#!/bin/bash

# this script tests the xz and normal gz compression

BIN="target/debug/rat"
if [ "$1" == "-d" ]; then
    BIN="rat"
    echo "[INFO] Using Debian package binary: $BIN"
else
    echo "[INFO] Using debug binary: $BIN"
fi

$BIN -x src/test_folder

# check if this file exists, test_folder.tar.xz
if [ -f "test_folder.tar.xz" ]; then
    echo "File test_folder.tar.xz exists."
else
    echo "File test_folder.tar.xz does not exist."
    exit 1
fi

$BIN test_folder.tar.xz # now try and uncompress it

if [ -d "test_folder" ]; then
    echo "Directory test_folder exists."
else
    echo "Directory test_folder does not exist."
    exit 1
fi

# --- This is for gz now ---

$BIN src/test_folder

# check if this file exists, test_folder.tar.gz
if [ -f "test_folder.tar.gz" ]; then
    echo "File test_folder.tar.gz exists."
else
    echo "File test_folder.tar.gz does not exist."
    exit 1
fi

$BIN test_folder.tar.gz # now try and uncompress it

if [ -d "test_folder" ]; then
    echo "Directory test_folder exists."
else
    echo "Directory test_folder does not exist."
    exit 1
fi

# delete all files
rm -rf test_folder.tar.gz test_folder test_folder.tar.xz