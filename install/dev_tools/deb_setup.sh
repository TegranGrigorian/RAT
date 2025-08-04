#!/bin/bash
# This script sets it up for debian packaging
# Move to the script's directory, then to the project root
cd "$(dirname "$0")"      # now in install/dev_tools
cd ../../           # now in project root (RAT)

cargo build --release
cp target/release/rat rat/usr/bin
dpkg-deb --build rat
sudo dpkg -i rat.deb