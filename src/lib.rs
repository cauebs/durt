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
/// Does not account twice the size of entries that are within other entries.
/// Implementation uses BTreeMap to build a path Trie.
pub fn calculate_unique_total_size(entries: &[Entry]) -> u64 {
    let mut filtered_entries = Vec::<&Entry>::new();
    let mut canonicalized_paths = Vec::<PathBuf>::new();

    // Canonicalize each path, silently ignoring failures.
    // TODO: Review if we should ignore failures.
    for entry in entries {
        if let Ok(path) = entry.path.canonicalize() {
            filtered_entries.push(entry);
            canonicalized_paths.push(path);
        }
    }

    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    struct TriePathNode {
        // Children nodes of this current path, accessed by path.
        children: BTreeMap<PathBuf, TriePathNode>,
        // Size of the file that ends at this node.
        node_size: u64,
    }

    let mut trie_root = TriePathNode {
        children: BTreeMap::new(),
        node_size: 0,
    };

    // For each entry/path, add it to the Trie if it wasn't already inserted.
    //
    // If the Trie receives a folder that is parent of a previously added file, then just consider
    // the parent folder, removing the childs, this way, we do not count them twice towards the
    // final total.
    for (i, entry) in filtered_entries.iter().enumerate() {
        let path = &canonicalized_paths[i];

        // Necessary because we need to check when it's the last path piece.
        let mut path_iter = path.iter().peekable();
        // Pointer to traverse the tree.
        let mut current_trie_node = &mut trie_root;
        // Size to be added at the endif the current entry isn't children of any other.
        let size_of_current_file = entry.size;

        while let Some(piece) = path_iter.next() {
            // Query for the node in the Trie which matches the current path piece.
            let entry = current_trie_node.children.entry(PathBuf::from(piece));

            // Keeps track if the current entry is child of another previously found.
            let mut already_considered = false;
            let next_trie_node = entry
                .and_modify(|_| {
                    // If we are in this block, it means that the node size was already considered
                    // because a parent of it was inserted. So we will skip this file.
                    already_considered = true;
                })
                // Add a node with 0 size, which may be changed after if it is the last piece.
                .or_insert(TriePathNode {
                    children: BTreeMap::new(),
                    node_size: 0,
                });

            // Skipping already accounted file, because it is nested inside of another one.
            if already_considered {
                break;
            }

            // If we are at the last piece of the current entry path, it means that this is the tip
            // that finally represents the file, and which path is the full file path.
            let is_the_last_piece = path_iter.peek().is_none();
            if is_the_last_piece {
                // Update the size of this piece.
                next_trie_node.node_size = size_of_current_file;
                // Drop all the childrens so that their sizes won't be added.
                next_trie_node.children.clear();
            }

            // Update the pointer to keep traversing the trie.
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
