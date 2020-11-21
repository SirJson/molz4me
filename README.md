# molz4me

A small wrapper around the molz4 to decompress and compress mozlz4 files. Nothing fancy.

## Usage

```bash
molz4me 0.1.0
Decompresses and compresses mozlz4 files found in Firefox

USAGE:
    molz4me.exe [OPTIONS] <INPUT> <OUPUT>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --color <color>          Colorize output [possible values: Never, Always, Ansi, Auto]
    -c, --compress <compress>    Compress instead of decompress the input (true/false)

ARGS:
    <INPUT>    The file to convert
    <OUPUT>    The desired output file
```
