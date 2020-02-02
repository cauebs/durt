use structopt::{
    clap::AppSettings::{ColoredHelp, DeriveDisplayOrder},
    StructOpt,
};

use std::path::PathBuf;

#[derive(StructOpt)]
#[structopt(settings = &[ColoredHelp, DeriveDisplayOrder])]
/// Command line tool for calculating the size of files and directories
pub struct Cli {
    /// Paths to files or directories. Use wildcards for recursion
    pub paths: Vec<PathBuf>,

    #[structopt(short = "b", long = "binary")]
    /// Use binary prefixes (Ki, Mi, Gi, etc.) instead of decimal
    pub use_binary_prefixes: bool,

    #[structopt(short = "P", long = "percentage")]
    /// Show each entry's percentage relative to the total
    pub show_percentages: bool,

    #[structopt(short, long = "min")]
    /// Ommit entries with size less than this
    pub minimum_percentage: Option<f64>,

    #[structopt(short = "t", long = "total")]
    /// Print the sum of all sizes at the end
    pub show_total: bool,

    #[structopt(short, long)]
    /// Print entries in ascending order of size
    pub sort: bool,

    #[structopt(short = "p", long = "by-path")]
    /// Sort by path instead of by size
    pub sort_by_path: bool,

    #[structopt(short, long = "reverse")]
    /// Reverse the order of the entries
    pub reverse_order: bool,
}
