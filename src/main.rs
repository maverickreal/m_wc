use clap::Parser;

#[derive(Parser)]
#[command(name="m_wc",
          about="Count lines, words, and bytes.")]
struct Cli {
    #[arg(short, long, help="Count bytes.")]
    bytes: bool,
    #[arg()]
    file_path: String
}

fn main () {
    let args = Cli::parse();
    println!("File: {}", args.file_path);
}
