use ansi_term::Colour;
use number_prefix::NumberPrefix;
use walkdir::WalkDir;

use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

trait ResultExt<T, E: Display> {
    fn log_err(self, path: Option<&Path>) -> Result<T, E>;
}

fn log_err<E: Display>(path: Option<&Path>, error: &E) {
    let message = match path {
        Some(path) => format!("{}: {}", path.display(), error),
        None => format!("{}", error),
    };
    eprintln!("{}", Colour::Red.paint(message));
}

impl<T, E: Display> ResultExt<T, E> for Result<T, E> {
    fn log_err(self, path: Option<&Path>) -> Result<T, E> {
        self.map_err(|error| {
            log_err(path, &error);
            error
        })
    }
}

pub struct Entry {
    pub path: PathBuf,
    pub size: u64,

    #[cfg(unix)]
    pub filesystem_id: u64,
}

impl Entry {
    pub fn from_path(path: &Path) -> Option<Entry> {
        #[cfg(unix)]
        let metadata = path.symlink_metadata().log_err(Some(path)).ok()?;

        let children = WalkDir::new(path).into_iter().filter_map(|entry| {
            entry
                .map_err(|error| {
                    if let Some(path) = error.path() {
                        log_err(Some(path), &error);
                    } else {
                        log_err(None, &error);
                    }
                })
                .ok()
        });

        let size = children
            .filter_map(|entry| entry.metadata().log_err(Some(entry.path())).ok())
            .map(|metadata| metadata.len())
            .sum();

        #[cfg(unix)]
        return Some(Entry {
            path: path.to_owned(),
            size,
            filesystem_id: metadata.dev(),
        });

        #[cfg(not(unix))]
        return Some(Entry {
            path: path.to_owned(),
            size,
        });
    }
}

pub fn format_size(size: u64, binary: bool) -> String {
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
