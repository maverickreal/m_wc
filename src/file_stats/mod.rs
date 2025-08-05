use std::{
    fs::{File, Metadata},
    io::{BufReader, Error, Read, stdin},
    str::{Utf8Error, from_utf8},
};

use crate::dao::{StatErrors, Stats};

pub trait DataStats {
    fn get_stats(source: &mut dyn Read) -> Result<Stats, Error> {
        let mut stats: Stats = Stats {
            bytes_count: 0,
            words_count: 0,
            lines_count: 0,
            chars_count: 0,
        };
        let mut buffer: [u8; 1024] = [0u8; 1024];
        let mut rem: Vec<u8> = Vec::new();
        let mut word_active: bool = false;
        let mut empty: bool = true;

        // Helper function to count whatever valid characters
        // available from the start in erroneous UTF-8 sequences.
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

        let mut reader = BufReader::new(source);

        loop {
            let bytes_read: usize = reader.read(&mut buffer)?;
            // update the words_count with the number of read bytres that are not whitesapce

            if bytes_read == 0 {
                // Handle any remaining bytes not processed.
                if !rem.is_empty() {
                    match from_utf8(&rem) {
                        Ok(utf_seq) => {
                            stats.chars_count += utf_seq.chars().count();
                        }
                        Err(err) => {
                            stats.chars_count += get_erroneous_part_count(&err, &rem);
                        }
                    }
                }

                break;
            }

            stats.bytes_count += bytes_read;
            empty = false;

            for byte in &buffer[0..bytes_read] {
                if *byte == b'\n' {
                    stats.lines_count += 1;
                }

                if !byte.is_ascii_whitespace() {
                    if !word_active {
                        stats.words_count += 1;
                        word_active = true;
                    }
                } else if word_active {
                    word_active = false;
                }
            }

            let mut rem_cpy: Vec<u8> = rem.clone();
            rem_cpy.extend_from_slice(&buffer[0..bytes_read]);

            match from_utf8(&rem_cpy) {
                Ok(utf_seq) => {
                    stats.chars_count += utf_seq.chars().count();
                    rem.clear();
                }

                Err(err) => {
                    let valid_upto: usize = err.valid_up_to();
                    stats.chars_count += get_erroneous_part_count(&err, &rem_cpy);
                    rem = rem_cpy[valid_upto..].to_vec();
                }
            }
        }

        if !empty && stats.lines_count == 0 {
            stats.lines_count = 1;
        }

        return Ok(stats);
    }
}

pub struct FileStats {
    pub(crate) stats: Stats,
}

pub struct StdInStats {
    pub(crate) stats: Stats,
}

impl DataStats for FileStats {}

impl FileStats {
    pub fn new(file_path: &str) -> (FileStats, StatErrors) {
        let file_result: Result<File, Error> = File::open(file_path);
        let no_fields_all_errors: (FileStats, StatErrors) = (
            FileStats {
                stats: Stats::new(),
            },
            StatErrors::new_all_fields_errors(),
        );

        if file_result.is_err() {
            return no_fields_all_errors;
        }

        let file: &mut File = &mut file_result.unwrap();
        let stat_result: Result<Stats, Error> = Self::get_stats(file);

        if stat_result.is_err() {
            let byte_count_result: Result<Metadata, Error> = file.metadata();

            if byte_count_result.is_err() {
                return no_fields_all_errors;
            }

            let byte_count: usize = byte_count_result.unwrap().len() as usize;
            let mut stats: Stats = Stats::new();
            stats.bytes_count = byte_count;
            let mut errors: StatErrors = StatErrors::new_all_fields_errors();
            errors.bytes_count = None;

            return (FileStats { stats }, errors);
        }

        return (
            FileStats {
                stats: stat_result.unwrap(),
            },
            StatErrors::new(),
        );
    }
}

impl DataStats for StdInStats {}

impl StdInStats {
    pub fn new() -> (StdInStats, StatErrors) {
        match Self::get_stats(&mut stdin()) {
            Ok(stats) => {
                return (StdInStats { stats }, StatErrors::new());
            }
            Err(err) => {
                eprintln!("{}", err);
                return (
                    StdInStats {
                        stats: Stats::new(),
                    },
                    StatErrors::new_all_fields_errors(),
                );
            }
        }
    }
}
