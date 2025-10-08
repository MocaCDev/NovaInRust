#![allow(dead_code, unused_imports, unreachable_patterns)]
use std::fmt;
use std::fs;
use std::io;
use std::io::Error;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct FileInfo {
    pub file_length: u64,
    pub file_data: String,
}

impl fmt::Display for FileInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\n{:?}\nFile Length: {}\nFile Data: {}\n",
            self, self.file_length, self.file_data
        )
    }
}

#[derive(Debug)]
pub enum ReadFileErrors {
    NoSuchDir(io::Error),
    NoSuchFile(PathBuf),
}

impl ReadFileErrors {
    pub fn print_error(&self) {
        match self {
            Self::NoSuchDir(io_err) => println!("(ReadFileErrors::NoSuchDir) {}", io_err),
            Self::NoSuchFile(err) => println!(
                "(ReadFileErrors::NoSuchFile) No Such File: {}",
                err.display()
            ),
            _ => println!("{:?}", self),
        }
    }
}

impl From<io::Error> for ReadFileErrors {
    fn from(value: io::Error) -> Self {
        ReadFileErrors::NoSuchDir(value)
    }
}

pub fn read_file(file_path: PathBuf) -> Result<FileInfo, ReadFileErrors> {
    let path = file_path.as_path().metadata()?;
    let file_length = path.len();

    let file_content = fs::read_to_string(file_path)?;

    return Ok(FileInfo {
        file_length: file_length,
        file_data: file_content,
    });
}
