<p align="center"><img src="src/design/horizontal.png" alt="durt" height="300px"></p>


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
                        (Sizes will be divided by 1024 instead of 1000)
    -P, --percentage    Show each item's percentage, relative to the total.
    -t, --total         Print the total at the end.
    -s, --sort          Print lines in ascending order.
                        (If --by-path is not passed, the size will be used)
    -p, --by-path       Sort the output lines by path, instead of by size.
    -r, --reverse       Reverse the order of the output lines.
    -h, --help          Prints help information
    -V, --version       Prints version information

OPTIONS:
    -m, --min <minimum_percentage>    Don't show items smaller than this.

ARGS:
    <paths>...    Path to files or directories

```

## Example
```
~/durt $ durt -st *
       534 B  Cargo.toml
     1.24 kB  README.md
    11.49 kB  Cargo.lock
    35.15 kB  LICENSE
    56.01 kB  src
   173.56 MB  target
  ---------
   173.66 MB

```
