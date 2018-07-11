use std::error::Error;
use std::io;
use std::path::{Path, PathBuf};

extern crate walkdir;
use walkdir::WalkDir;

#[macro_use]
extern crate structopt;
use structopt::clap::AppSettings::{ArgRequiredElseHelp, ColoredHelp, DeriveDisplayOrder};
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
#[structopt(raw(
    settings = "&[ColoredHelp, ArgRequiredElseHelp, DeriveDisplayOrder]",
))]
struct Opt {
    /// Path to files or directories
    #[structopt(parse(from_os_str))]
    paths: Vec<PathBuf>,

    /// Use binary prefixes (KiB, MiB, GiB, etc).
    /// {n}Sizes will be divided by 1024 instead of 1000.
    #[structopt(short = "b", long = "binary")]
    use_binary_prefixes: bool,

    /// Show the percentage for each item, relative to the total.
    #[structopt(short = "P", long = "percentage")]
    show_percentages: bool,

    /// Only show items with at least <minimum_percentage>
    #[structopt(short = "m", long = "min")]
    minimum_percentage: Option<f64>,

    /// Print the total at the end.
    #[structopt(short = "t", long = "total")]
    show_total: bool,

    /// Print lines in ascending order.
    /// {n}If --by-path is not passed, the size will be used.
    #[structopt(short = "s", long = "sort")]
    sort: bool,

    /// Sort the output lines by path, instead of by size.
    #[structopt(short = "p", long = "by-path")]
    sort_by_path: bool,

    /// Reverse the order of the output lines.
    #[structopt(short = "r", long = "reverse")]
    reverse_order: bool,
}

struct Entry {
    path: PathBuf,
    size: u64,
}

fn format_size(size: u64, binary: bool) -> String {
    let size = size as f64;

    let prefixed = if binary {
        binary_prefix(size)
    } else {
        decimal_prefix(size)
    };

    let formatted = match prefixed {
        Standalone(s) => format!("{} B", s as u64),
        Prefixed(p, s) => format!("{:.2} {}B", s, p),
    };

    if binary {
        format!(" {:>10}", formatted)
    } else {
        format!(" {:>9}", formatted)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    let mut entries = Vec::new();
    let mut total_size = 0;

    for path in opt.paths {
        let size = recursive_size(&path)?;
        total_size += size;
        entries.push(Entry { path, size });
    }

    if opt.sort {
        if opt.sort_by_path {
            entries.sort_by_key(|e| e.path.clone());
        } else {
            entries.sort_by_key(|e| e.size);
        }
    }

    if opt.reverse_order {
        entries.reverse();
    }

    for entry in entries.iter() {
        let percentage = 100.0 * entry.size as f64 / total_size as f64;

        match opt.minimum_percentage {
            Some(p) if percentage < p => continue,
            _ => (),
        }

        print!("{}  ", format_size(entry.size, opt.use_binary_prefixes));
        if opt.show_percentages {
            print!("({})  ", format!("{:>5.2}%", percentage));
        }
        println!("{:>3}", entry.path.display());
    }

    if opt.show_total {
        println!(" {}", "-".repeat(if opt.use_binary_prefixes { 10 } else { 9 }));
        println!("{}", format_size(total_size, opt.use_binary_prefixes));
    }

    Ok(())
}
