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

ARGS:
    <paths>...    Paths to files or directories. Use wildcards for recursion

FLAGS:
    -b, --binary        Use binary prefixes (Ki, Mi, Gi, etc.) instead of decimal
    -P, --percentage    Show each entry's percentage relative to the total
    -t, --total         Print the sum of all sizes at the end
    -s, --sort          Print entries in ascending order of size
    -p, --by-path       Sort by path instead of by size
    -r, --reverse       Reverse the order of the entries
    -h, --help          Prints help information
    -V, --version       Prints version information

OPTIONS:
    -m, --min <minimum-percentage>    Omit entries with size less than this
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
