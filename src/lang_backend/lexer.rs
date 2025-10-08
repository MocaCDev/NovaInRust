use super::read_file::{FileInfo, ReadFileErrors, read_file};
use std::mem;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Lexer {
    file_length: u64,
    file_data: String,
    current_index: usize,
    current_char: char,
}

#[derive(Debug)]
pub enum Tokens {
    VariableDeclaration,
}

#[derive(Debug)]
pub struct Token {
    pub token_id: Tokens,
    pub token_value: String,
}

#[derive(Debug)]
pub enum LexerErrors {
    InvalidToken(String),
}

impl Lexer {
    pub fn new(file_path: PathBuf) -> Result<Box<Lexer>, ReadFileErrors> {
        let file_info: FileInfo = match read_file(file_path) {
            Ok(data) => data,
            Err(err) => {
                err.print_error();
                return Err(err);
            }
        };

        let mut lexer: Box<Lexer> = Box::new(Lexer {
            file_length: file_info.file_length,
            file_data: file_info.file_data.clone(),
            current_index: 0,
            current_char: '\0',
        });

        mem::drop(file_info);

        lexer.current_char = lexer.file_data.chars().nth(lexer.current_index).unwrap();

        return Ok(lexer);
    }

    fn skip_whitespace(&mut self) {
        while (self.current_char == ' ' || self.current_char == '\t' || self.current_char == '\n') {
            self.current_index += 1;
            self.current_char = self.file_data.chars().nth(self.current_index).unwrap();
        }
    }

    pub fn get_next_token(&mut self) -> Result<Token, LexerErrors> {
        match self.current_char {
            '/' => {
                /* Comment. */
                while (self.current_char != '\n') {
                    self.current_index += 1;
                    self.current_char = self.file_data.chars().nth(self.current_index).unwrap();
                }

                self.get_next_token()
            }
            ' ' | '\t' | '\n' => {
                self.skip_whitespace();
                self.get_next_token()
            }
            '@' => {
                /* Variable declaration */
                return Ok(Token {
                    token_id: Tokens::VariableDeclaration,
                    token_value: String::from("@"),
                });
            }
            _ => {
                /* Other Stuff */
                return Err(LexerErrors::InvalidToken(format!(
                    "Unkown character: {}",
                    self.current_char
                )));
            }
        }
    }
}
