mod display;
mod entry;
mod listing;

use std::path::Path;

use clap::Parser;

use display::{display_long, display_short};
use listing::{list_entries, sort_entries};

#[derive(Parser, Debug)]
#[command(name = "ft_ls", version, about = "List all files in directory like unix")]
struct Args {
    #[arg(default_value = ".")]
    destination: Vec<String>,
    #[arg(short = 'l', long)]
    long_format: bool,
    #[arg(short = 'a', long)]
    show_hidden: bool,
    #[arg(short = 'r', long)]
    rev: bool,
    #[arg(short = 't', long)]
    order_by_update_date: bool,
    #[arg(short = 'R', long)]
    recursive: bool,
}

fn run(args: &Args, path: &Path, show_header: bool) {
    if show_header {
        println!("\n{}:", path.display());
    }

    let mut entries = list_entries(path, args.show_hidden);
    sort_entries(&mut entries, args.order_by_update_date, args.rev);

    if args.long_format {
        display_long(&entries);
    } else {
        display_short(&entries);
    }

    if args.recursive {
        for entry in &entries {
            if entry.is_dir() && entry.name != "." && entry.name != ".." {
                run(args, &entry.path, true);
            }
        }
    }
}

fn main() {
    let args = Args::parse();
    let multiple = args.destination.len() > 1;

    for dest in &args.destination {
        let path = Path::new(dest);
        if !path.exists() {
            eprintln!("ft_ls: cannot access '{}': No such file or directory", dest);
            continue;
        }
        run(&args, path, multiple || args.recursive);
    }
}
