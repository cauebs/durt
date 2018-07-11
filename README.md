# durt
Command line tool for calculating the size of files and directories

## Installation
```
~ $ cargo install durt
```

## Usage
```
USAGE:
    durt [FLAGS] [OPTIONS] [paths]...

FLAGS:
    -b, --binary        Use binary prefixes (KiB, MiB, GiB, etc).
                        Sizes will be divided by 1024 instead of 1000.
    -P, --percentage    Show the percentage for each item, relative to the total.
    -t, --total         Print the total at the end.
    -s, --sort          Print lines in ascending order.
                        If --by-path is not passed, the size will be used.
    -p, --by-path       Sort the output lines by path, instead of by size.
    -r, --reverse       Reverse the order of the output lines.
    -h, --help          Prints help information
    -V, --version       Prints version information

OPTIONS:
    -m, --min <minimum_percentage>    Only show items with at least <minimum_percentage>

ARGS:
    <paths>...    Path to files or directories
```

## Example
```
~/durt $ durt -st *
     478 B  Cargo.toml
     908 B  README.md
   7.08 kB  src
   9.79 kB  Cargo.lock
  35.15 kB  LICENSE
 569.59 MB  target
 ---------
 569.64 MB
```
