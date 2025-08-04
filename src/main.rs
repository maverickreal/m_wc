mod file_stats;

use clap::Parser;
use file_stats::FileStats;

/// Command line interface for the word count tool.
#[derive(Parser)]
#[clap(name = "m_wc", about = "Count lines, words, and bytes.")]
struct Cli {
    /// The path to the file to be analyzed.
    #[arg(index = 1, required = true)]
    file_path: String,

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
    let file_path: String = args.file_path;

    match FileStats::new(&file_path) {
        Ok(mut file) => {
            // Default case.
            let print_all: bool = !args.lines && !args.words && !args.bytes && !args.chars;

            // Print the number of bytes if requested.
            if args.bytes || print_all {
                match file.get_num_of_bytes() {
                    Ok(byte_count) => {
                        println!("{}", byte_count);
                    }
                    Err(err) => {
                        println!("Error parsing the metadata of the file!");
                        eprintln!("{}", err);
                    }
                }
            }

            // Print the number of words if requested.
            if args.words | print_all {
                match file.get_num_of_words() {
                    Ok(word_count) => {
                        println!("{}", word_count);
                    }
                    Err(err) => {
                        println!("Error reading the file!");
                        eprintln!("{}", err);
                    }
                }
            }

            // Print the number of lines if requested.
            if args.lines || print_all {
                match file.get_num_of_lines() {
                    Ok(line_count) => {
                        println!("{}", line_count);
                    }
                    Err(err) => {
                        println!("Error reading the file!");
                        eprintln!("{}", err);
                    }
                }
            }

            // Print the number of characters if requested.
            if args.chars || print_all {
                match file.get_num_of_chars() {
                    Ok(char_count) => {
                        println!("{}", char_count);
                    }
                    Err(err) => {
                        println!("Error reading the file!");
                        eprintln!("{}", err);
                    }
                }
            }
        }

        // Handle the error if the file could not be opened.
        Err(err) => {
            println!("Error opening the file at the provided path!");
            eprintln!("{}", err);
        }
    }
}

