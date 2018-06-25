# durt

## Installation
```
~ $ cargo install --git https://github.com/cauebs/durt
```

## Usage
```
USAGE:
    durt [FLAGS] [paths]...

FLAGS:
    -b, --binary     Use binary prefixes (KiB, MiB, GiB, etc). Sizes will be divided by 1024 instead of 1000.
    -p, --by-path    Sort the output lines by path, instead of by size.
    -h, --help       Prints help information
    -r, --reverse    Reverse the order of the output lines.
    -s, --sort       Print lines in ascending order. If --by-path is not passed, the size will be used.
    -V, --version    Prints version information

ARGS:
    <paths>...    Path to files or directories
```

## Example
```
~/durt $ durt * -s
          168 B Cargo.toml
          816 B README.md
        6.47 KB src
        9.41 KB Cargo.lock
      206.24 MB target
```

## Disclaimer
This was just an experiment and probably won't be maintained.
For long term usage, you might want to try [Dust](https://github.com/bootandy/dust/).
