mod dao;
mod file_stats;

use crate::dao::{DataSource, StatErrors};
use clap::Parser;
use file_stats::{DataStats, FileStats, StdInStats};

/// Command line interface for the word count tool.
#[derive(Parser)]
#[clap(name = "m_wc", about = "Count lines, words, and bytes.")]
struct Cli {
    /// The path to the file to be analyzed.
    #[arg(index = 1)]
    file_path: Option<String>,

    /// Flag to count bytes in the file.
    #[arg(short, long)]
    bytes: bool,

    /// Flag to count words in the file.
    #[arg(short, long)]
    words: bool,

    /// Flag to count lines in the file.
    #[arg(short, long)]
    lines: bool,

    /// Flag to count characters in the file.
    #[arg(short, long)]
    chars: bool,
}

fn main() {
    // Parse command line arguments.
    let args: Cli = Cli::parse();
    let (source_stats, errors) = match args.file_path {
        Some(file_path) => {
            let (s, e) = FileStats::new(&file_path);
            (DataSource::File(s), e)
        }
        None => {
            let (s, e) = StdInStats::new();
            (DataSource::StdIn(s), e)
        }
    };

    // let (source_stats, errors): (FileStats, StatErrors) = FileStats::new(&args.file_path.unwrap());
    // let (source_stats, errors): (StdInStats, StatErrors) = StdInStats::new();
    let print_all: bool = !args.lines && !args.words && !args.bytes && !args.chars;

    if args.bytes || print_all {
        println!("Bytes data:\n{}", source_stats.stats().bytes_count);
        println!("{}", errors.bytes_count.unwrap_or_default());
    }

    if args.chars || print_all {
        println!("Characters data:\n{}", source_stats.stats().chars_count);
        println!("{}", errors.chars_count.unwrap_or_default());
    }

    if args.words || print_all {
        println!("Words data:\n{}", source_stats.stats().words_count);
        println!("{}", errors.words_count.unwrap_or_default());
    }

    if args.lines || print_all {
        println!("Lines data:\n{}", source_stats.stats().lines_count);
        println!("{}", errors.lines_count.unwrap_or_default());
    }
}
