use structopt::{
    clap::AppSettings::{ArgRequiredElseHelp, ColoredHelp, DeriveDisplayOrder},
    StructOpt,
};

use std::path::PathBuf;

#[derive(StructOpt)]
#[structopt(raw(settings = "&[ColoredHelp, ArgRequiredElseHelp, DeriveDisplayOrder]"))]
pub struct Cli {
    /// Path to files or directories
    #[structopt(parse(from_os_str))]
    pub paths: Vec<PathBuf>,

    /// Use binary prefixes (KiB, MiB, GiB, etc).
    /// {n}(Sizes will be divided by 1024 instead of 1000)
    #[structopt(short = "b", long = "binary")]
    pub use_binary_prefixes: bool,

    /// Show each item's percentage, relative to the total.
    #[structopt(short = "P", long = "percentage")]
    pub show_percentages: bool,

    /// Don't show items smaller than this.
    #[structopt(short = "m", long = "min")]
    pub minimum_percentage: Option<f64>,

    /// Print the total at the end.
    #[structopt(short = "t", long = "total")]
    pub show_total: bool,

    /// Print lines in ascending order.
    /// {n}(If --by-path is not passed, the size will be used)
    #[structopt(short = "s", long = "sort")]
    pub sort: bool,

    /// Sort the output lines by path, instead of by size.
    #[structopt(short = "p", long = "by-path")]
    pub sort_by_path: bool,

    /// Reverse the order of the output lines.
    #[structopt(short = "r", long = "reverse")]
    pub reverse_order: bool,
}
