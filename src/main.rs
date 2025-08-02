use clap::Parser;
use std::{
    fs,
    io::{self, BufReader, Read},
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

struct FileStats {
    file: fs::File,
}

impl FileStats {
    /// Creates a new `FileStats` instance.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be opened.
    pub fn new(file_path: &str) -> Result<Self, io::Error> {
        let file = fs::File::open(file_path)?;
        Ok(FileStats { file })
    }

    /// Returns the number of bytes in the file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file metadata cannot be read.
    pub fn get_num_of_bytes(&self) -> Result<u64, io::Error> {
        let byte_count = self.file.metadata()?.len();
        Ok(byte_count)
    }

    /// Returns the number of words in the file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read.
    pub fn get_num_of_words(&self) -> Result<u64, io::Error> {
        let mut word_count: u64 = 0;
        let mut word_active: bool = false;

        let reader = io::BufReader::new(&self.file);

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

    /// Returns the number of lines in the file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read.
    pub fn get_num_of_lines(&self) -> Result<u64, io::Error> {
        let mut line_count: u64 = 0;
        let mut empty: bool = true;
        let reader = io::BufReader::new(&self.file);

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

    /// Returns the number of characters in the file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or if the file contains
    /// invalid UTF-8.
    pub fn get_num_of_chars(&self) -> Result<u64, io::Error> {
        let mut char_cnt: usize = 0;
        let mut buffer = [0u8; 1024];
        let mut rem: Vec<u8> = Vec::new();
        let mut reader = BufReader::new(&self.file);

        loop {
            let bytes_read = reader.read(&mut buffer)?;

            if bytes_read == 0 {
                if !rem.is_empty() {
                    if str::from_utf8(&rem).is_ok() {
                        char_cnt += rem.iter().count();
                    } else {
                        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8"));
                    }
                }

                break;
            }

            let mut rem_cpy = rem.clone();
            rem_cpy.extend_from_slice(&buffer[0..bytes_read]);

            match str::from_utf8(&rem_cpy) {
                Ok(utf_seq) => {
                    char_cnt += utf_seq.chars().count();
                    rem.clear();
                }

                Err(err) => {
                    let valid_upto = err.valid_up_to();

                    if valid_upto > 0 {
                        char_cnt += str::from_utf8(&rem_cpy[0..valid_upto])
                            .unwrap()
                            .chars()
                            .count();
                    }

                    rem = rem[valid_upto..].to_vec();
                }
            }
        }

        return Ok(char_cnt as u64);
    }
}

fn main() {
    let args = Cli::parse();
    let file_path = args.file_path;
    let file = FileStats::new(&file_path);

    match file {
        Ok(file) => {
            let print_all = !args.lines && !args.words && !args.bytes && !args.chars;
            let print_bytes = args.bytes || print_all;
            let print_words = args.words || print_all;
            let print_lines = args.lines || print_all;
            let print_chars = args.chars || print_all;

            if print_bytes {
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

            if print_words {
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

            if print_lines {
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

            if print_chars {
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

        Err(err) => {
            println!("Error opening the file at the provided path!");
            eprintln!("{}", err);
        }
    }
}
