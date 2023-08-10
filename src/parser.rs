use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Parser {
    file: BufReader<File>,
    current_command: String,
    next_command: String,
}

impl Parser {
    fn new(f: &str) -> Parser {
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
    fn set_command(&mut self) -> bool {
        loop {
            // Attempt to read the next line from the input file.
            let bytes = self
                .file
                .read_line(&mut self.next_command)
                .unwrap_or(0usize);

            if bytes > 0 {
                // Split the line by '/' to remove comments and other unnecessary data.
                let to_verified: Vec<String> = self
                    .next_command
                    .clone()
                    .split('/')
                    .map(String::from)
                    .collect();

                // Trim the line and take the first part as the verified next instruction.
                self.next_command = to_verified[0].clone().trim().to_string();

                // Skip lines that are comments (start with '/').
                if self.next_command.chars().next().unwrap_or('/') == '/' {
                    continue;
                }
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
