# durt

## Installation
```
~ $ cargo install durt
```

## Usage
```
USAGE:
    durt [FLAGS] [paths]...

FLAGS:
    -b, --binary     Use binary prefixes (kiB, MiB, GiB, etc). Sizes will be divided by 1024 instead of 1000.
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
~/durt $ durt -s *
      473 B  Cargo.toml
      934 B  README.md
    1.08 kB  LICENSE
    6.55 kB  src
    9.79 kB  Cargo.lock
  232.46 MB  target
```
