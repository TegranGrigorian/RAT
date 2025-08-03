#!bin/bash
# Install RAT and move it to the usr / bin directory
# Usage: ./install.sh
# Check if the script is run as root
if [ "$(id -u)" -ne 0 ]; then
    echo "This script must be run as root. Use sudo or switch to the root user."
    exit 1
fi
# Check if the RAT file exists
if [ ! -f "rat" ]; then
    echo "RAT file not found. Please ensure the RAT file is in the current directory."
    exit 1
fi
# Move the RAT file to /usr/bin
mv rat /usr/bin/rat
# Check if the move was successful
if [ $? -ne 0 ]; then
    echo "Failed to move the RAT file to /usr/bin. Please check your permissions."
    exit 1
fi
# Make the RAT file executable
chmod +x /usr/bin/rat
# Check if the chmod was successful
if [ $? -ne 0 ]; then
    echo "Failed to make the RAT file executable. Please check your permissions."
    exit 1
fi
rat --help
echo "RAT installed successfully. You can now run it using the command 'rat'."