use ansi_term::Colour;
use number_prefix::NumberPrefix;
use walkdir::WalkDir;

use std::{
    collections::BTreeMap,
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

#[derive(PartialEq, Eq, PartialOrd, Ord)]
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

/// Calculate the sum of sizes of all entries
///
/// Ignore nested files when calculating the total
///
/// For the nested files:
///  - `folder/         (5 MB)`
///  - `folder/big_file (15 MB)`
///
/// The is 15 MB instead of 20 MB because the inner file is inside of the
/// folder that was also received as an argument
///
/// Implemented with the Trie data structure, made of HashMap and PathBufs
/// that represent each path components of the canonicalized file paths
pub fn calculate_unique_total_size(entries: &[Entry]) -> u64 {
    // Entries, but with with canonicalized paths
    let entries = {
        let mut new_entries: Vec<(PathBuf, &Entry)> = vec![];

        for entry in entries {
            // Log errors and ignore them in the total sum
            let canonical_path = entry.path.canonicalize().log_err(Some(&entry.path));
            if let Ok(path) = canonical_path {
                new_entries.push((path, entry));
            }
        }
        new_entries
    };

    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    struct TriePathNode {
        // Children nodes of this current path, accessed by path
        children: BTreeMap<PathBuf, TriePathNode>,
        // Size of the file that ends at this node
        node_size: u64,
    }

    let mut trie_root = TriePathNode {
        children: BTreeMap::new(),
        node_size: 0,
    };

    // For each entry/path, add it to the Trie if it wasn't already inserted
    //
    // If the Trie receives a folder that is parent of a previously added file, then just consider
    // the parent folder, removing the childs, this way, we do not count them twice towards the
    // final total
    for (path, entry) in entries {
        // Necessary because we need to check when it's the last path piece
        let mut path_iter = path.iter().peekable();
        // Pointer to traverse the tree
        let mut current_trie_node = &mut trie_root;
        // Size to be added at the end if the current entry isn't children of any other
        let size_of_current_file = entry.size;

        while let Some(piece) = path_iter.next() {
            // Query for the node in the Trie which matches the current path piece
            let entry = current_trie_node.children.entry(PathBuf::from(piece));

            let mut is_current_node_size_zero = true;
            // Keeps track if the current entry is child of another previously found
            let next_trie_node = entry
                .and_modify(|next_node| {
                    // If we are in this block, it means that this node was already present in the
                    // trie tree
                    is_current_node_size_zero = next_node.node_size == 0;
                })
                // Add a node with 0 size, which is only changed afterwards if it's the last piece
                .or_insert(TriePathNode {
                    children: BTreeMap::new(),
                    node_size: 0,
                });

            // Skipping current entry, because it's nested inside an already accounted file, or is
            // a repeated file
            if !is_current_node_size_zero {
                break;
            }

            // If we are at the last piece of the current entry path, it means that this is the tip
            // that finally represents the file, and which path is the full file path
            let is_the_last_piece = path_iter.peek().is_none();
            if is_the_last_piece {
                // Update the size of the last trie node for this piece
                next_trie_node.node_size = size_of_current_file;
                // Drop all the childrens so that their sizes won't be added twice
                next_trie_node.children.clear();
            }

            // Update the pointer to keep traversing the trie
            current_trie_node = next_trie_node;
        }
    }

    fn trie_recursive_sum(node: &TriePathNode) -> u64 {
        let children_sum: u64 = node.children.values().map(trie_recursive_sum).sum();
        node.node_size + children_sum
    }

    // Traverse the trie tree to calculate the sum
    trie_recursive_sum(&trie_root)
}
