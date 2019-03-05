use ansi_term::Colour;
use number_prefix::{binary_prefix, decimal_prefix, Prefixed, Standalone};
use walkdir::WalkDir;

use std::path::Path;

macro_rules! unwrap_or_log {
    ($x:expr) => {
        match $x {
            Ok(x) => x,
            Err(e) => {
                let message = format!("{}", e);
                let message = Colour::Red.paint(message);
                eprintln!("{}", message);
                return None;
            }
        }
    };
}

pub fn measure_recursive(path: &Path) -> u64 {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| {
            let entry = unwrap_or_log!(entry);
            let path = entry.path();

            // this avoids following symlinks
            let metadata = unwrap_or_log!(path.symlink_metadata());
            Some(metadata.len())
        })
        .sum()
}

pub fn format_decimal(size: u64) -> String {
    let prefixed = decimal_prefix(size as f32);
    format(prefixed)
}

pub fn format_binary(size: u64) -> String {
    let prefixed = binary_prefix(size as f32);
    format(prefixed)
}

fn format(prefixed: number_prefix::Result<f32>) -> String {
    match prefixed {
        Standalone(number) => format!("{} B", number as u64),
        Prefixed(prefix, number) => format!("{:.2} {}B", number, prefix),
    }
}
