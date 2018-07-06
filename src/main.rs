use std::io;
use std::path::{Path, PathBuf};
use std::error::Error;

extern crate walkdir;
use walkdir::WalkDir;

#[macro_use]
extern crate structopt;
use structopt::clap::AppSettings::{ArgRequiredElseHelp, ColoredHelp};
use structopt::StructOpt;

extern crate number_prefix;
use number_prefix::{binary_prefix, decimal_prefix, Prefixed, Standalone};

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

struct Entry {
    path: PathBuf,
    size: u64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    let mut entries = Vec::new();
    for path in opt.paths {
        let size = recursive_size(&path)?;
        entries.push(Entry { path, size });
    }

    if opt.sort {
        if opt.by_path {
            entries.sort_by_key(|e| e.path.clone());
        } else {
            entries.sort_by_key(|e| e.size);
        }
    }

    if opt.reverse {
        entries.reverse();
    }

    let formatter_function = if opt.binary {
        binary_prefix
    } else {
        decimal_prefix
    };

    for entry in entries {
        let formatted_size = match formatter_function(entry.size as f64) {
            Standalone(s) => format!("{:.2}  B", s),
            Prefixed(p, s) => format!("{:.2} {}B", s, p),
        };

        println!(" {:>10} {}", formatted_size, entry.path.display());
    }

    Ok(())
}
