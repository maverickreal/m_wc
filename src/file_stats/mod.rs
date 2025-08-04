use std::{
    fs::{self, File},
    io::{BufReader, Error, Read, Seek, SeekFrom},
    str::{Utf8Error, from_utf8},
};

/// A structure to hold a file and provide statistics about its contents.
pub(crate) struct FileStats {
    file: fs::File,
}

impl FileStats {
    /// Creates a new instance of `FileStats` by opening the file at the given path.
    ///
    /// # Arguments
    ///
    /// * `file_path` - A string slice that holds the path of the file.
    ///
    /// # Returns
    ///
    /// * `Result<Self, Error>` - On success, returns a `FileStats` instance. On failure, returns an I/O error.
    pub fn new(file_path: &str) -> Result<Self, Error> {
        let file: File = fs::File::open(file_path)?;
        return Ok(FileStats { file });
    }

    /// Calculates the number of bytes in the file.
    ///
    /// # Returns
    ///
    /// * `Result<u64, Error>` - On success, returns the byte count. On failure, returns an I/O error.
    pub fn get_num_of_bytes(&self) -> Result<u64, Error> {
        let byte_count: u64 = self.file.metadata()?.len();
        return Ok(byte_count);
    }

    /// Calculates the number of words in the file.
    ///
    /// # Returns
    ///
    /// * `Result<u64, Error>` - On success, returns the word count. On failure, returns an I/O error.
    pub fn get_num_of_words(&mut self) -> Result<u64, Error> {
        let mut word_count: u64 = 0;
        let mut word_active: bool = false;

        // Reset file cursor to the start.
        self.file.seek(SeekFrom::Start(0))?;

        let reader: BufReader<&File> = BufReader::new(&self.file);

        // Iterate through each byte in the file.
        for byte in reader.bytes() {
            match byte {
                Ok(_byte) => {
                    // Check if the byte is not whitespace, indicating a word.
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

        return Ok(word_count);
    }

    /// Calculates the number of lines in the file.
    ///
    /// # Returns
    ///
    /// * `Result<u64, Error>` - On success, returns the line count. On failure, returns an I/O error.
    pub fn get_num_of_lines(&mut self) -> Result<u64, Error> {
        let mut line_count: u64 = 0;
        let mut empty: bool = true;

        // Reset file cursor to the start.
        self.file.seek(SeekFrom::Start(0))?;

        let reader: BufReader<&File> = BufReader::new(&self.file);

        // Iterate through each byte in the file.
        for byte in reader.bytes() {
            match byte {
                Ok(_byte) => {
                    empty = false;

                    // Check for newline character.
                    if _byte == b'\n' {
                        line_count += 1;
                    }
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }

        // If file is not empty but no lines are counted, count as one line.
        if !empty && line_count == 0 {
            line_count = 1;
        }

        return Ok(line_count);
    }

    /// Calculates the number of characters in the file.
    ///
    /// # Returns
    ///
    /// * `Result<u64, Error>` - On success, returns the character count. On failure, returns an I/O error.
    pub fn get_num_of_chars(&mut self) -> Result<u64, Error> {
        let mut char_cnt: usize = 0;
        let mut buffer: [u8; 1024] = [0u8; 1024];
        let mut rem: Vec<u8> = Vec::new();

        // Helper function to count characters in erroneous UTF-8 sequences.
        let get_erroneous_part_count = |err: &Utf8Error, _rem: &Vec<u8>| -> usize {
            let valid_upto: usize = err.valid_up_to();

            match from_utf8(&_rem[0..valid_upto]) {
                Ok(utf_seq) => {
                    return utf_seq.chars().count();
                }
                Err(err) => {
                    eprintln!("{}", err);
                    return 0;
                }
            }
        };

        // Reset file cursor to the start.
        self.file.seek(SeekFrom::Start(0))?;

        let mut reader: BufReader<&File> = BufReader::new(&self.file);

        loop {
            let bytes_read: usize = reader.read(&mut buffer)?;

            if bytes_read == 0 {
                // Handle any remaining bytes not processed.
                if !rem.is_empty() {
                    match from_utf8(&rem) {
                        Ok(utf_seq) => {
                            char_cnt += utf_seq.chars().count();
                        }
                        Err(err) => {
                            char_cnt += get_erroneous_part_count(&err, &rem);
                        }
                    }
                }

                break;
            }

            let mut rem_cpy: Vec<u8> = rem.clone();
            rem_cpy.extend_from_slice(&buffer[0..bytes_read]);

            match from_utf8(&rem_cpy) {
                Ok(utf_seq) => {
                    char_cnt += utf_seq.chars().count();
                    rem.clear();
                }

                Err(err) => {
                    let valid_upto: usize = err.valid_up_to();
                    char_cnt += get_erroneous_part_count(&err, &rem_cpy);
                    rem = rem_cpy[valid_upto..].to_vec();
                }
            }
        }

        return Ok(char_cnt as u64);
    }
}
