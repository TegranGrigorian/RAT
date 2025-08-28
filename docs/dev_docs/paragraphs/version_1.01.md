# V1.0.1

Date: August 28, 2025

Update: Added tar.xz compability and more

## Message:
Hello!

I hope you are doing well! This update will consists of compatibility with tar.xz file types. By passing the -x flag, we can compress a folder into a tar.xz file. Furthermore, I modified the workflofw code to make it easier to expand onto different tar file formats.

Here are the patch notes, thank you for your attention!


## Patch Notes

1. Added xz compatibility
* use "-x" flag when attempting to compress a folder into .tar.gz
* just like tar.gz, no flag required to unzip a .tar.xz file
    * you can always delete the file with a -d flag!
2. Modified workflow and services
* Abstraction in main to reference a workflow in the rat-service file.
    * This should allow for easier updates for different file types without adding much code
3. Cleaned up managment utilites.
4. Added more docs.

## Whats next?
1. More file format compatibility
* normal tar, and others
    * normal tar flag = -t
    * .tgz = -g
    * .7z = -z
2. Adding more "ease of use" flags
* These flags will be for outputting everything and other important operations a user might need to use.
3. Potential Windows Port
* Unlikley

## Final Notes
Once again, thank you for you attention and use of this program. If you have an issue, add a post on the "Issues" tab on github.

Thank you again for your time, have a good day!

Tegran Grigorian