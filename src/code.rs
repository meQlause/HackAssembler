use super::table::Table;
use std::fs::File;
use std::io::Write;

pub trait CodeTrait {
    fn new(output: String) -> Code;
    fn a_instruction(&self, a: i16) -> String;
    fn c_instruction(&self, a: &String, table: &Table) -> String;
    fn write(&mut self, text: &str);
}
pub struct Code {
    pub file: File,
}

impl CodeTrait for Code {
    fn new(output: String) -> Code {
        Code {
            file: File::create(&output).expect("can't create file"),
        }
    }
    /// Converts the given `i16` value to a binary string representation.
    ///
    /// # Arguments
    ///
    /// - `a`: An `i16` value to be converted to a binary string.
    ///
    /// # Example
    ///
    /// ```
    /// let genie = Genie;
    /// let binary_string = genie.a_instruction(42);
    /// assert_eq!(binary_string, "000000000010101");
    /// ```
    ///
    /// This function takes an `i16` value as input and returns a string representation of the value
    /// in binary format. The returned string will have leading zeros to ensure a total width of 16 characters.
    fn a_instruction(&self, a: i16) -> String {
        // println!("0{:015b} = {}", a, a);
        format!("0{:015b}", a)
    }
    /// Converts the given C-instruction components into a binary representation.
    ///
    /// # Arguments
    ///
    /// - `a`: A reference to a `String` representing the C-instruction.
    /// - `table`: A reference to a `Table` containing the mapping of C-instruction components to their binary representations.
    ///
    /// # Example
    ///
    /// ```
    /// let genie = Genie;
    /// let table = Table::new();
    /// let binary_string = genie.c_instruction(&"D=A".to_string(), &table);
    /// assert_eq!(binary_string, "1110110000010000");
    /// ```
    ///
    /// This function takes a reference to a `String` representing a C-instruction and a reference to a `Table`
    /// which contains the mapping of C-instruction components (destination, computation, and jump) to their binary representations.
    /// The function processes each component of the C-instruction and generates the corresponding binary representation.
    /// It then concatenates the binary representations of the components and returns the final formatted binary string.
    fn c_instruction(&self, a: &String, table: &Table) -> String {
        let (mut jmp, mut dst, mut comp, mut r, mut to_return) = (
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::from("111"),
        );

        let ins: Vec<String> = a
            .split(&['=', ';'])
            .into_iter()
            .map(|f| f.trim().to_string())
            .collect();
        let con: Vec<String> = vec![
            "JGT".to_string(),
            "JEQ".to_string(),
            "JGE".to_string(),
            "JLT".to_string(),
            "JNE".to_string(),
            "JLE".to_string(),
            "JMP".to_string(),
        ];

        // Process each component of the computation instruction.

        for (index, value) in ins.iter().enumerate() {
            let key = value.clone();
            if index == 0 {
                if let Some(v) = table.get_dst(&key) {
                    dst += v;
                }
            }
            if index == 1 && con.contains(&key.to_string()) {
                dst.clear();
                dst += "000";
                to_return += "0";
                if let Some(v) = table.get_a0(r.trim()) {
                    comp += v;
                }

                if let Some(v) = table.get_jmp(&key) {
                    jmp += v;
                }
                break;
            } else if index == 1 {
                jmp += "000";
                // println!("{:?}", a1.get(key));
                if let Some(v) = table.get_a1(&key) {
                    comp += v;
                    to_return += "1";
                } else {
                    to_return += "0";
                    if let Some(v) = table.get_a0(&key) {
                        comp += v;
                    }
                }
            }

            if let Some(v) = table.get_jmp(&key) {
                jmp.clear();
                jmp += v;
            }
            // Update the 'r' variable for the next iteration
            // This is necessary if the line of code does contain JUMP value and doesn't provide both comp and dest value, so we can set those to value by this variable
            r = key.to_string();
        }

        // Return the final formatted binary representation.
        // println!("{}{}{}{} = {:?}", to_return, comp, dst, jmp, ins.join("="));
        to_return + &comp + &dst + &jmp
    }

    /// Writes the given text to the file.
    ///
    /// # Arguments
    ///
    /// - `text`: A string slice representing the text to be written to the file.
    ///
    /// # Example
    ///
    /// ```
    /// let genie = Genie::new("output.txt");
    /// genie.write("Hello, world!");
    /// ```
    ///
    /// This method takes a string slice (`&str`) representing the text to be written to the file.
    /// It uses the `writeln!` macro to write the text to the file, appending a newline character at the end.
    /// If the write operation fails, it will panic with the message "can't write."
    fn write(&mut self, text: &str) {
        writeln!(self.file, "{}", text).expect("can't write.");
    }
}
