use structopt::StructOpt;
use tabular::{Row, Table};

use std::path::PathBuf;
use wild;

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

    let cli = Cli::from_iter(wild::args_os());

    if cli.paths.is_empty() {
        Cli::clap().print_help().unwrap();
        return;
    }

    let mut entries = cli
        .paths
        .into_iter()
        .filter_map(|path| {
            if !path.exists() {
                None
            } else {
                let size = size::measure_recursive(&path);
                Some(Entry { path, size })
            }
        })
        .collect::<Vec<_>>();

    if cli.sort {
        if cli.sort_by_path {
            entries.sort_by(|a, b| a.path.cmp(&b.path));
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

    let total_size = entries.iter().map(|e| e.size).sum();

    for entry in entries {
        let percentage = entry.size as f64 / total_size as f64 * 100.0;

        if let Some(m) = cli.minimum_percentage {
            if percentage < m {
                continue;
            }
        }

        let formatted_size = size::format(entry.size, cli.use_binary_prefixes);
        let mut row = Row::new().with_cell(formatted_size);

        if cli.show_percentages {
            row.add_cell(format!("({:>5.2}%)", percentage));
        }

        row.add_cell(entry.path.display());
        table.add_row(row);
    }

    if cli.show_total {
        let separator = "-".repeat(if cli.use_binary_prefixes { 10 } else { 9 });
        table.add_heading(" ".repeat(2) + &separator);

        let formatted_total = size::format(total_size, cli.use_binary_prefixes);
        let mut row = Row::new().with_cell(formatted_total);

        if cli.show_percentages {
            row.add_cell("");
        }

        row.add_cell("(total)");
        table.add_row(row);
    }

    print!("{}", table);
}
