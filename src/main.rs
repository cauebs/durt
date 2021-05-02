use clap::{Clap, IntoApp};
use tabular::{Row, Table};
use wild;

mod cli;
use cli::Cli;
use durt::{calculate_unique_total_size, format_size, Entry};

fn main() {
    #[cfg(windows)]
    ansi_term::enable_ansi_support().expect("Failed to enable ANSI support.");

    let cli = Cli::parse_from(wild::args_os());

    if cli.paths.is_empty() {
        Cli::into_app().print_help().unwrap();
        return;
    }

    let all_entries = cli.paths.iter().filter_map(|path| Entry::from_path(&path));
    let mut entries: Vec<Entry>;

    #[cfg(unix)]
    {
        entries = if cli.same_fs {
            let mut all_entries = all_entries.peekable();
            let first_fs = match all_entries.peek() {
                Some(first) => first.filesystem_id,
                None => return,
            };

            all_entries
                .filter(|e| e.filesystem_id == first_fs)
                .collect()
        } else {
            all_entries.collect()
        };
    }

    #[cfg(not(unix))]
    {
        entries = all_entries.collect::<Vec<_>>();
    }

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

    let total_size = calculate_unique_total_size(&entries);
    let mut omitted_entries = 0;

    for entry in entries {
        let percentage = entry.size as f64 / total_size as f64 * 100.0;

        if let Some(m) = cli.minimum_percentage {
            if percentage < m {
                omitted_entries += 1;
                continue;
            }
        }

        let formatted_size = format_size(entry.size, cli.use_binary_prefixes);
        let mut row = Row::new().with_cell(formatted_size);

        if cli.show_percentages {
            row.add_cell(format!("({:>5.2}%)", percentage));
        }

        row.add_cell(entry.path.display());
        table.add_row(row);
    }

    if omitted_entries > 0 {
        let mut row = Row::new().with_cell("");

        if cli.show_percentages {
            row.add_cell("");
        }

        row.add_cell(if omitted_entries == 1 {
            format!("({} entry omitted)", omitted_entries)
        } else {
            format!("({} entries omitted)", omitted_entries)
        });

        table.add_row(row);
    }

    if cli.show_total {
        let separator = "-".repeat(if cli.use_binary_prefixes { 10 } else { 9 });
        let mut row = Row::new().with_cell(separator);

        if cli.show_percentages {
            row.add_cell("");
        }

        table.add_row(row.with_cell(""));

        let formatted_total = format_size(total_size, cli.use_binary_prefixes);
        let mut row = Row::new().with_cell(formatted_total);

        if cli.show_percentages {
            row.add_cell("");
        }

        row.add_cell("(total)");
        table.add_row(row);
    }

    print!("{}", table);
}
