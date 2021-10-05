use clap::{
    AppSettings::{ColoredHelp, DeriveDisplayOrder},
    Clap,
};

use std::path::PathBuf;

#[derive(Clap)]
#[clap(setting = ColoredHelp, setting = DeriveDisplayOrder)]
/// Command line tool for calculating the size of files and directories
pub struct Cli {
    /// Paths to files or directories. Use wildcards for recursion
    pub paths: Vec<PathBuf>,

    #[clap(short = 'b', long = "binary")]
    /// Use binary prefixes (Ki, Mi, Gi, etc.) instead of decimal
    pub use_binary_prefixes: bool,

    #[clap(short = 'P', long = "percentage")]
    /// Show each entry's percentage relative to the total
    pub show_percentages: bool,

    #[clap(short, long = "min")]
    /// Omit entries with size less than this
    pub minimum_percentage: Option<f64>,

    #[clap(short = 't', long = "total")]
    /// Print the sum of all sizes at the end
    pub show_total: bool,

    #[clap(short, long)]
    /// Print entries in ascending order of size
    pub sort: bool,

    #[clap(short = 'p', long = "by-path")]
    /// Sort by path instead of by size
    pub sort_by_path: bool,

    #[clap(short, long = "reverse")]
    /// Reverse the order of the entries
    pub reverse_order: bool,

    #[cfg(unix)]
    #[clap(short = 'f', long)]
    /// Ignore entries from filesystems different from that of the first path passed
    pub same_fs: bool,
}
