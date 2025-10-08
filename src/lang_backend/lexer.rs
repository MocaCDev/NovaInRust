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
    VariableName,
    Equal,
    StringVal,
}

#[derive(Debug)]
pub struct Token<'a> {
    pub token_id: Tokens,
    pub token_value: &'a str,
}

#[derive(Debug)]
pub enum LexerErrors {
    InvalidToken(String),
}

impl Lexer {
    pub fn new<'a>(file_path: PathBuf) -> Result<Box<Lexer>, ReadFileErrors> {
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
        while self.current_char == ' ' || self.current_char == '\t' || self.current_char == '\n' {
            self.current_index += 1;
            self.current_char = self.file_data.chars().nth(self.current_index).unwrap();
        }
    }

    fn is_ascii(&self) -> bool {
        (self.current_char as u8 >= 0x61 && self.current_char as u8 <= 0x7A)
            || (self.current_char as u8 >= 0x41 && self.current_char as u8 <= 0x5A)
    }

    pub fn get_next_token<'a>(&'a mut self) -> Result<Token<'a>, LexerErrors> {
        let mut start_index: usize = self.current_index;

        match self.current_char {
            '/' => {
                /* Comment. */
                while self.current_char != '\n' {
                    self.current_index += 1;
                    self.current_char = self.file_data.chars().nth(self.current_index).unwrap();
                }

                self.get_next_token()
            }
            ' ' | '\t' | '\n' => {
                self.skip_whitespace();
                self.get_next_token()
            }
            '=' => {
                let tok: Token = Token {
                    token_id: Tokens::Equal,
                    token_value: &self
                        .file_data
                        .get(start_index..self.current_index + 1)
                        .unwrap(),
                };

                self.current_index += 1;
                self.current_char = self.file_data.chars().nth(self.current_index).unwrap();
                return Ok(tok);
            }
            '\"' => {
                self.current_index += 1;
                self.current_char = self.file_data.chars().nth(self.current_index).unwrap();
                start_index = self.current_index;

                while self.current_char != '\"' {
                    self.current_index += 1;
                    self.current_char = self.file_data.chars().nth(self.current_index).unwrap();
                }

                let tok: Token = Token {
                    token_id: Tokens::StringVal,
                    token_value: &self
                        .file_data
                        .get(start_index..self.current_index + 1)
                        .unwrap(),
                };

                self.current_index += 1;
                self.current_char = self.file_data.chars().nth(self.current_index).unwrap();
                return Ok(tok);
            }
            '@' => {
                /* Variable declaration */
                let tok: Token = Token {
                    token_id: Tokens::VariableDeclaration,
                    token_value: &self
                        .file_data
                        .get(start_index..self.current_index + 1)
                        .unwrap(),
                };

                self.current_index += 1;
                self.current_char = self.file_data.chars().nth(self.current_index).unwrap();
                return Ok(tok);
            }
            _ => {
                /* Other Stuff */
                if self.is_ascii() {
                    while self.current_char != ' '
                        && self.current_char != '\n'
                        && self.current_char != '\t'
                    {
                        self.current_index += 1;
                        self.current_char = self.file_data.chars().nth(self.current_index).unwrap();
                    }

                    let tok: Token = Token {
                        token_id: Tokens::VariableName,
                        token_value: &self
                            .file_data
                            .get(start_index..self.current_index + 1)
                            .unwrap(),
                    };

                    return Ok(tok);
                }

                return Err(LexerErrors::InvalidToken(format!(
                    "Unkown character: {}",
                    self.current_char
                )));
            }
        }
    }
}
