use std::io;
use std::path::{Path, PathBuf};

extern crate walkdir;
use walkdir::WalkDir;

#[macro_use]
extern crate structopt;
use structopt::clap::AppSettings::{ArgRequiredElseHelp, ColoredHelp};
use structopt::StructOpt;

extern crate humansize;
use humansize::file_size_opts::{BINARY, DECIMAL};
use humansize::FileSize;

fn recursive_size(path: &Path) -> io::Result<u64> {
    let mut total_size = 0;

    for entry in WalkDir::new(path) {
        let entry = entry?;
        let path = entry.path();

        // we need to avoid following symlinks here
        let metadata = path.symlink_metadata()?;
        let size = metadata.len();

        total_size += size;
    }

    Ok(total_size)
}

#[derive(StructOpt)]
#[structopt(raw(setting = "ColoredHelp", setting = "ArgRequiredElseHelp"))]
struct Opt {
    /// Path to files or directories
    #[structopt(parse(from_os_str))]
    paths: Vec<PathBuf>,

    /// Use binary prefixes (KiB, MiB, GiB, etc).
    /// Sizes will be divided by 1024 instead of 1000.
    #[structopt(short = "b", long = "binary")]
    binary: bool,

    /// Print lines in ascending order.
    /// If --by-path is not passed, the size will be used.
    #[structopt(short = "s", long = "sort")]
    sort: bool,

    /// Sort the output lines by path, instead of by size.
    #[structopt(short = "p", long = "by-path")]
    by_path: bool,

    /// Reverse the order of the output lines.
    #[structopt(short = "r", long = "reverse")]
    reverse: bool,
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();
    let num_paths = opt.paths.len();

    let mut data = Vec::new();
    for path in opt.paths {
        let size = recursive_size(&path)?;
        data.push((path, size))
    }

    if opt.sort {
        if opt.by_path {
            data.sort();
        } else {
            data.sort_by_key(|t| t.1);
        }
    }

    if opt.reverse {
        data.reverse();
    }

    for (path, size) in data {
        // it's safe to unwrap here because the will never be negative
        let printable_size = if opt.binary {
            size.file_size(BINARY).unwrap()
        } else {
            size.file_size(DECIMAL).unwrap()
        };

        if num_paths == 1 {
            println!("{}", printable_size);
        } else {
            println!("{:>15} {}", printable_size, path.display());
        }
    }

    Ok(())
}
