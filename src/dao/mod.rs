use crate::file_stats::{FileStats, StdInStats};

pub struct Stats {
    pub(crate) bytes_count: usize,
    pub(crate) words_count: usize,
    pub(crate) lines_count: usize,
    pub(crate) chars_count: usize,
}

impl Stats {
    /// Creates a new instance of `Stats` with all fields set to 0.
    pub fn new() -> Stats {
        Stats {
            bytes_count: 0,
            words_count: 0,
            lines_count: 0,
            chars_count: 0,
        }
    }
}

pub struct StatErrors {
    pub(crate) bytes_count: Option<String>,
    pub(crate) words_count: Option<String>,
    pub(crate) lines_count: Option<String>,
    pub(crate) chars_count: Option<String>,
}

impl StatErrors {
    pub fn new_all_fields_errors() -> Self {
        StatErrors {
            bytes_count: Some(String::from(
                "An error occured during computing bytes count!",
            )),
            words_count: Some(String::from(
                "An error occured during computing words count!",
            )),
            lines_count: Some(String::from(
                "An error occured during computing lines count!",
            )),
            chars_count: Some(String::from(
                "An error occured during computing chars count!",
            )),
        }
    }

    pub fn new() -> Self {
        StatErrors {
            bytes_count: None,
            words_count: None,
            lines_count: None,
            chars_count: None,
        }
    }
}

pub enum DataSource {
    File(FileStats),
    StdIn(StdInStats),
}

impl DataSource {
    pub fn stats(&self) -> &Stats {
        match self {
            DataSource::File(file_stats) => &file_stats.stats,
            DataSource::StdIn(stdin_stats) => &stdin_stats.stats,
        }
    }
}
