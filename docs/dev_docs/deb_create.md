# Steps to Create a .deb Package for the RAT Project

## 1. Ensure the Directory Structure
Organize your project files into the following structure:
```
rat/
├── DEBIAN/
│   └── control
└── usr/
    └── local/
        └── bin/
            └── rat
```
- **DEBIAN/control**: Contains metadata about the package.
- **usr/local/bin/rat**: Place the compiled binary here.

## 2. Create the `control` File
The `control` file should look like this:
```yaml
Package: rat
Version: 0.1.0
Section: utils
Priority: optional
Architecture: amd64
Maintainer: Tegran Grigorian <tegrgrigoralt@gmail.com>
Description: A easy to use abstraction of the TAR package format. This package provides a simple interface to create and extract TAR archives. It is designed to be easy to use and integrate into other applications.
```
- **Package**: Name of the package.
- **Version**: Version of the package.
- **Architecture**: Use `amd64` for 64-bit systems.
- **Maintainer**: Your name and email.
- **Description**: A brief description of the package.

## 3. Build the Package
Run the following command to build the `.deb` package:
```bash
dpkg-deb --build rat
```
- This command creates a `.deb` file from the `rat/` directory.

## 4. Install the Package
To install the generated `.deb` file, use:
```bash
sudo dpkg -i rat.deb
```
- This installs the package on your system.

## 5. Test the Installation
Verify that the `rat` command is available:
```bash
rat --help
```
- This ensures the package was installed correctly and is functional.

## Notes
- Ensure the `control` file ends with a newline.
- If you encounter issues, check the `dpkg-deb` error messages for guidance.

## file strucute of deb

```
rat/
├── DEBIAN
│   └── control
└── usr
    └── bin
        └── rat
```