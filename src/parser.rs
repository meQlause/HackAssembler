use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::{Seek, SeekFrom};

pub struct Parser {
    pub file: BufReader<File>,
    pub current_command: String,
    next_command: String,
}

impl Parser {
    pub fn new(f: &str) -> Parser {
        match File::open(f) {
            Ok(f) => {
                let to_pass = BufReader::new(f);
                return Parser {
                    file: to_pass,
                    current_command: String::new(),
                    next_command: String::new(),
                };
            }
            _ => panic!("Can't open file"),
        }
    }
    pub fn set_command(&mut self) -> bool {
        loop {
            self.next_command.clear();
            // Attempt to read the next line from the input file.
            let bytes = self
                .file
                .read_line(&mut self.next_command)
                .unwrap_or(0usize);
            if bytes > 0 {
                // Skip lines that are comments (start with '/').
                if self.next_command.chars().next() == Some('/')
                    || self.next_command.trim().as_bytes().len() == 0
                {
                    continue;
                }
                // Split the line by '/' to remove comments and other unnecessary data.
                let to_verified: Vec<String> = self
                    .next_command
                    .clone()
                    .split('/')
                    .map(String::from)
                    .collect();

                // Trim the line and take the first part as the verified next instruction.
                self.next_command = to_verified[0].clone().trim().to_string();

                // If a valid command is found, set current_command by invoke advance() method with nextinstruction and return true.
                self.advance();
                return true;
            } else {
                // If no bytes are read, the end of the file is reached, so return false.
                return false;
            };
        }
    }
    fn advance(&mut self) {
        // Update current_command with the next_instruction.
        self.current_command = self.next_command.clone();
    }
}
