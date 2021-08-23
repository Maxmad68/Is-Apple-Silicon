# Is-Apple-Silicon
Quick command-line tool to check if a program is compatible with Apple Silicon chips.

![](https://madrau.fr/ISAS-Github/screen1.png)

## Installation
Download the file in release, and copy it to the directory `/usr/local/bin`
If the program is immediately killed when executed, execute the following command:

    sudo xattr -cr /usr/local/bin/is_apple_silicon


## Usage
`is_apple_silicon` just takes as arguments the names or the paths of programs to test. It can handle multiple programs.
