#![allow(dead_code, unused_imports)]
mod lang_backend;

use lang_backend::lexer::{Lexer, Token};
use lang_backend::read_file::{FileInfo, ReadFileErrors, read_file};
use std::env;
use std::fmt;
use std::mem;
use std::path::PathBuf;

#[derive(Debug)]
struct TestStruct {
    name: String,
}

impl fmt::Display for TestStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}, {}", self, self.name)
    }
}

fn main() {
    let d: TestStruct = TestStruct {
        name: String::from("Bob"),
    };

    println!("{}", d);
    println!("{}", env::current_dir().expect("Nah").display());

    /*let file_data: FileInfo = match read_file(PathBuf::from("test.nva")) {
        Ok(data) => data,
        Err(e) => {
            e.print_error();
            return;
        }
    };

    println!("{}", file_data);*/
    let mut lexer: Box<Lexer> = match Lexer::new(PathBuf::from("test.nva")) {
        Ok(lexer) => lexer,
        Err(err) => {
            err.print_error();
            return;
        }
    };

    let tokenInfo: Token = match lexer.get_next_token() {
        Ok(token) => token,
        Err(err) => {
            println!("{:?}", err);
            mem::drop(lexer);
            return;
        }
    };
    println!("{:?}", tokenInfo);
}
