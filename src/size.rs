use ansi_term::Colour;
use number_prefix::NumberPrefix;
use walkdir::{DirEntry, WalkDir};

use std::{error::Error, fmt::Display, path::Path};

fn log_error<E: Display>(error: E) {
    let message = Colour::Red.paint(format!("{}", error));
    eprintln!("{}", message);
}

pub fn measure_recursive(path: &Path) -> Option<u64> {
    if path.exists() {
        Some(WalkDir::new(path).into_iter().filter_map(measure).sum())
    } else {
        log_error(format!("No such file or directory: {}", path.display()));
        None
    }
}

fn measure(entry: Result<DirEntry, impl Error>) -> Option<u64> {
    let entry = entry.map_err(log_error).ok()?;
    let metadata = entry.path().symlink_metadata().map_err(log_error).ok()?;
    Some(metadata.len())
}

pub fn format(size: u64, binary: bool) -> String {
    let formatted = if binary {
        NumberPrefix::binary(size as f64)
    } else {
        NumberPrefix::decimal(size as f64)
    };

    match formatted {
        NumberPrefix::Standalone(number) => {
            let padding = if binary { "   " } else { "  " };
            format!("{}{}B", number as u64, padding)
        }
        NumberPrefix::Prefixed(prefix, number) => format!("{:.2} {}B", number, prefix),
    }
}
