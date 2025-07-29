use clap::Parser;
use std::{
    fs,
    io::{BufReader, Read},
};

#[derive(Parser)]
#[command(name = "m_wc", about = "Count lines, words, and bytes.")]
struct Cli {
    #[arg(index = 1, required = true)]
    file_path: String,

    #[arg(short, long)]
    bytes: bool,

    #[arg(short, long)]
    words: bool,

    #[arg(short, long)]
    lines: bool,

    #[arg(short, long)]
    chars: bool,
}

struct FileMetaDataExtract {
    file: fs::File,
}

impl FileMetaDataExtract {
    fn new(_file: fs::File) -> Self {
        FileMetaDataExtract { file: _file };
    }

    fn get_num_of_bytes(&self) -> io::Result<u64> {
        let byte_count = self.file.metadata()?.len();
        Ok(byte_count)
    }

    fn get_num_of_chars(&self) -> io::Result<u64> {}

    fn get_num_of_words(&self) -> io::Result<u64> {
        let mut word_count: u64 = 0;
        let mut word_active: bool = false;

        let reader = BufReader::new(&self.file);

        for byte in reader.bytes() {
            match byte {
                Ok(_byte) => {
                    if !_byte.is_ascii_whitespace() {
                        if !word_active {
                            word_count += 1;
                            word_active = true;
                        }
                    } else if word_active {
                        word_active = false;
                    }
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }

        Ok(word_count)
    }

    fn get_num_of_lines(&self) -> io::Result<u64> {
        let mut line_count: u64 = 0;
        let mut empty: bool = true;
        let reader = BufReader::new(&self.file);

        for byte in reader.bytes() {
            match byte {
                Ok(_byte) => {
                    empty = false;

                    if _byte == b'\n' {
                        line_count += 1;
                    }
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }

        if !empty && line_count == 0 {
            line_count = 1;
        }

        Ok(line_count)
    }
}

fn main() {
    let args = Cli::parse();
    let file_path = args.file_path;
    let file = fs::File::open(&file_path);

    match file {
        Ok(file) => {
            let print_all = !args.lines && !args.words && !args.bytes && !args.chars;
            let print_bytes = args.bytes || print_all;
            let print_words = args.words || print_all;
            let print_lines = args.lines || print_all;
            let print_chars = args.chars || print_all;

            

            if print_all {

            } else if 

            let num_of_bytes = file.metadata();

            match num_of_bytes {
                Ok(byte_count) => {
                    println!("{}", byte_count.len());
                }
                Err(err) => {
                    println!("Error parsing the metadata of the file!");
                    eprintln!("{}", err);
                }
            };
        }

        Err(err) => {
            println!("Error opening the file at the provided path!");
            eprintln!("{}", err);
        }
    }
}
