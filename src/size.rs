use ansi_term::Colour;
use number_prefix::{NumberPrefix, Prefixed, Standalone};
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

pub fn format(size: u64, binary: bool) -> String {
    let formatted = if binary {
        NumberPrefix::binary(size as f64)
    } else {
        NumberPrefix::decimal(size as f64)
    };

    match formatted {
        Standalone(number) => {
            let padding = if binary { "   " } else { "  " };
            format!("{}{}B", number as u64, padding)
        }
        Prefixed(prefix, number) => format!("{:.2} {}B", number, prefix),
    }
}
