use structopt::StructOpt;
use tabular::{Row, Table};

use std::path::PathBuf;

mod size;

mod cli;
use cli::Cli;

struct Entry {
    path: PathBuf,
    size: u64,
}

fn main() {
    #[cfg(windows)]
    ansi_term::enable_ansi_support();

    let cli = Cli::from_args();

    let mut entries = cli
        .paths
        .into_iter()
        .filter_map(|path| {
            let size = size::measure_recursive(&path);

            if !path.exists() {
                return None;
            }

            Some(Entry { path, size })
        })
        .collect::<Vec<_>>();

    let total_size = entries.iter().map(|e| e.size).sum();

    if cli.sort {
        if cli.sort_by_path {
            entries.sort_by_key(|e| e.path.to_owned());
        } else {
            entries.sort_by_key(|e| e.size);
        }
    }

    if cli.reverse_order {
        entries.reverse();
    }

    let mut table = if cli.show_percentages {
        Table::new("  {:>}  {:>}  {:<}")
    } else {
        Table::new("  {:>}  {:<}")
    };

    for entry in entries {
        let percentage = entry.size as f64 / total_size as f64 * 100.0;

        if let Some(m) = cli.minimum_percentage {
            if percentage < m {
                continue;
            }
        }

        let formatted_size = if cli.use_binary_prefixes {
            size::format_binary(entry.size)
        } else {
            size::format_decimal(entry.size)
        };

        let mut row = Row::new().with_cell(formatted_size);

        if cli.show_percentages {
            row.add_cell(format!("({:>5.2}%)", percentage));
        }

        row.add_cell(entry.path.display());
        table.add_row(row);
    }

    if cli.show_total {
        let separator = "-".repeat(if cli.use_binary_prefixes { 10 } else { 9 });
        table.add_heading(" ".repeat(3) + &separator);

        let formatted_total = if cli.use_binary_prefixes {
            size::format_binary(total_size)
        } else {
            size::format_decimal(total_size)
        };

        let mut row = Row::new().with_cell(formatted_total);
        for _ in 0..table.column_count() - 1 {
            row.add_cell("");
        }

        table.add_row(row);
    }

    println!("{}", table);
}
