use std::fs::File;
use std::io::{BufRead, BufReader};

pub trait ParseTrait {
    fn new(name: &str) -> Parser;
    fn set_command(&mut self) -> bool;
}
trait ParseTraitPrivate {
    fn advance(&mut self);
}
pub struct Parser {
    pub file: BufReader<File>,
    pub current_command: String,
    next_command: String,
}

impl ParseTrait for Parser {
    /// Creates a new Parser instance by opening the specified file.
    ///
    /// # Arguments
    ///
    /// - `f`: A string slice representing the file path to open.
    ///
    /// # Example
    ///
    /// ```
    /// let parser = Parser::new("commands.txt");
    /// ```
    ///
    /// This method takes a string slice (`&str`) representing the file path to open.
    /// It attempts to open the file using `File::open` and returns a new `Parser` instance.
    /// If the file open operation is successful, the method initializes the `Parser` struct with the opened file.
    /// The opened file is wrapped in a buffered reader using `BufReader::new`.
    /// The `current_command` and `next_command` fields of the `Parser` struct are initialized with empty strings.
    /// If the file open operation fails, the method panics with the message "Can't open file".
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
    /// Sets the current command by reading the next line from the input file.
    ///
    /// This method attempts to read the next line from the input file and processes it to set the current command.
    /// It skips lines that are comments (start with '/') or empty lines.
    /// The line is split by '/' to remove comments and other unnecessary data.
    /// The first part of the line (after removing comments) is taken as the verified next instruction.
    /// If a valid command is found, the `advance()` method is invoked with the next instruction to set the current command.
    ///
    /// # Returns
    ///
    /// - `true` if a valid command is found and the current command is successfully set.
    /// - `false` if the end of the file is reached and no new command is available.
    ///
    /// # Example
    ///
    /// ```
    /// let mut parser = Parser::new("commands.txt");
    /// let result = parser.set_command();
    /// assert_eq!(result, true);
    /// ```
    fn set_command(&mut self) -> bool {
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
}

impl ParseTraitPrivate for Parser {
    fn advance(&mut self) {
        // Update current_command with the next_instruction.
        self.current_command = self.next_command.clone();
    }
}
