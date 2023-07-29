use std::collections::HashMap;
use std::default::Default;
use std::fs::File;
use std::io::{BufRead, BufReader, Result, Seek, SeekFrom, Write};

/// `FileOrError` is an enum representing either an open file handle (`File`) or an error message (`String`).
/// This enum is used in the context of the `HackAssembler` struct to handle the input file.
/// It allows capturing both the successfully opened file and any potential errors that occurred during file operations.
#[derive(Debug)]
pub enum FileOrError {
    /// Represents an open file handle (`File`) after successful file opening.
    F(File),

    /// Represents an error message (`String`) if the file could not be opened or read.
    E(String),
}

/// The `HackAssembler` struct represents a simple Hack Assembly language assembler. It allows you to load Hack Assembly code from a file, process the instructions, and generate corresponding machine code. The assembler supports both built-in instructions and user-defined labels/symbols for memory addresses. The machine code is represented as binary strings.
///
/// # Examples
///
/// ```
/// // Create a new instance of the HackAssembler
/// let mut assembler = HackAssembler::default();
///
/// // Load the Hack Assembly code from the file "program.asm"
/// assembler.load_file(&String::from("program.asm"));
///
/// // Assemble the loaded code and get the resulting machine code to the test.hack file
/// match assembler.assemble(&"test.hack".to_string()) {
///     Ok(_) => {
///         println!("Converted.");
///     }
///     Err(error_msg) => {
///         println!("Error: {}", error_msg);
///     }
/// }
/// }
/// ```
pub struct HackAssembler {
    /// The name of the input file containing the Hack Assembly code.
    file_name: String,

    /// A file handle that represents the opened input file or an error if the file could not be opened or read.
    file: FileOrError,

    /// A nested HashMap representing the Hack Assembly instructions. The outer HashMap's key is the mnemonic
    /// (e.g., "A-M" or "0;JMP"), and the inner HashMap contains key-value pairs for each field (e.g., "dest",
    /// "comp", "jump") of the instruction and its corresponding binary representation.
    instructions: HashMap<String, HashMap<String, String>>,

    /// A HashMap that stores the mapping between labels/symbols found in the Hack Assembly code and their
    /// memory addresses (represented as i16-bit signed integers).
    labels_symbols_map: HashMap<String, i16>,
}

impl HackAssembler {
    /// Displays the current state of the `HackAssembler` instance by printing the values of its fields.
    /// This method is mainly used for debugging purposes.
    // pub fn display(&self) {
    //     dbg!("{}", &self.file_name);
    //     dbg!("{}", &self.file);
    //     dbg!("{}", &self.instructions);
    //     dbg!("{}", &self.labels_symbols_map);
    // }

    /// Loads an input file containing Hack Assembly code into the `HackAssembler`.
    /// This method updates the `file_name` field and attempts to open the specified file.
    /// If the file is successfully opened, it adds labels and symbols to the `labels_symbols_map`.
    ///
    /// # Arguments
    ///
    /// * `file` - A reference to a `String` containing the name of the input file to be loaded.
    pub fn load_file(&mut self, file: &String) {
        self.file_name = file.to_string();
        self.file = match File::open(file) {
            Ok(file) => FileOrError::F(file),
            Err(_) => FileOrError::E(String::from("Can't load file")),
        };
        if let &FileOrError::F(_) = &self.file {
            self.add_labels_symbols_map();
        };
    }
    fn a_instruction(&self, a: &i16) -> String {
        println!("0{:015b} = {}", a, a);
        format!("0{:015b}", a)
    }
    fn c_instruction(&self, a: &String) -> String {
        let (mut jmp, mut dst, mut comp, mut r, mut to_return) = (
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::from("111"),
        );

        let (a1, a0, dest, jump) = (
            self.instructions.get("COMP_A_1").unwrap(),
            self.instructions.get("COMP_A_0").unwrap(),
            self.instructions.get("DEST").unwrap(),
            self.instructions.get("JUMP").unwrap(),
        );

        let re_ins: Vec<&str> = a.split(&['/']).collect();
        let binding = re_ins[0].to_string();
        let mut ins: Vec<&str> = binding.split(&['=', ';']).collect();
        ins = ins.iter().map(|f| f.trim()).collect();
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

        // println!("{:?}", ins);
        for (index, value) in ins.into_iter().enumerate() {
            let key = value.clone().trim();
            if index == 0 {
                if let Some(v) = dest.get(key) {
                    dst += v;
                }
            }
            if index == 1 && con.contains(&key.to_string()) {
                dst.clear();
                dst += "000";
                to_return += "0";
                if let Some(v) = a0.get(r.trim()) {
                    comp += v;
                }

                if let Some(v) = jump.get(key) {
                    jmp += v;
                }
                break;
            } else if index == 1 {
                jmp += "000";
                // println!("{:?}", a1.get(key));
                if let Some(v) = a1.get(key) {
                    comp += v;
                    to_return += "1";
                } else {
                    to_return += "0";
                    if let Some(v) = a0.get(key) {
                        comp += v;
                    }
                }
            }

            if let Some(v) = jump.get(key) {
                jmp.clear();
                jmp += v;
            }
            // Update the 'r' variable for the next iteration
            // This is necessary if the line of code does contain JUMP value and doesn't provide both comp and dest value, so we can set those to value by this variable
            r = key.to_string();
        }

        // Return the final formatted binary representation.
        println!("{}{}{}{} = {}", to_return, comp, dst, jmp, a);
        to_return + &comp + &dst + &jmp
    }
    /// Internal method to add labels and symbols to the `labels_symbols_map`.
    /// This method is called during file loading to build the mapping between labels/symbols and memory addresses.
    fn add_labels_symbols_map(&mut self) {
        let (mut l, mut l_sum, mut n) = (0, 0, 15);

        match &self.file {
            FileOrError::F(f) => {
                let mut new_f = f.clone();
                // First iteration
                let reader = BufReader::new(new_f);
                for line in reader.lines() {
                    let mut label = line.unwrap().trim().to_string();
                    // dbg!("label = {}", &label);
                    if let Some(c) = label.chars().next() {
                        if c == '(' {
                            // println!("added {}", label);
                            label = label[1..label.len() - 1].to_string();
                            self.labels_symbols_map.insert(label, l - l_sum);
                            l_sum += 1;
                        }
                        if c != '/' && c != ' ' {
                            l += 1;
                        }
                    }
                }
                //Reset the pointer to 0
                new_f.seek(SeekFrom::Start(0)).unwrap();

                // Second Iteration
                let reader = BufReader::new(new_f);
                for line in reader.lines() {
                    let mut symbol = line.unwrap().trim().to_string();
                    // dbg!("symbol ={}", &symbol);
                    if let Some(c) = symbol.chars().next() {
                        if c == '@' {
                            symbol = symbol.trim()[1..].to_string();
                            match symbol.parse::<i16>() {
                                Ok(_) => continue,
                                Err(_) => {
                                    if !self.labels_symbols_map.contains_key(&symbol) {
                                        // println!("added {}", symbol);

                                        n += 1;
                                        self.labels_symbols_map.entry(symbol).or_insert(n);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            FileOrError::E(e) => {
                panic!("{}", e);
            }
        }
    }

    pub fn assemble(&self, file: &String) -> Result<()> {
        let mut file_to_write = File::create(file)?;
        match &self.file {
            FileOrError::F(file) => {
                //Reset the pointer to 0
                let mut new_f = file.clone();
                new_f.seek(SeekFrom::Start(0)).unwrap();

                let reader = BufReader::new(new_f);
                for line in reader.lines() {
                    let input = line.unwrap().trim().to_string();
                    if let Some(c) = input.chars().next() {
                        if c == '@' {
                            let value: i16 = if let Some(val) =
                                self.labels_symbols_map.get(&input[1..].to_string())
                            {
                                *val
                            } else {
                                let val = input.clone().trim()[1..].to_string();
                                let to_return = val.parse::<i16>().unwrap();
                                to_return
                            };

                            writeln!(file_to_write, "{}", self.a_instruction(&value))?;
                        } else if c != '/' && c != ' ' && c != '(' {
                            writeln!(file_to_write, "{}", self.c_instruction(&input))?;
                        }
                    }
                }
            }
            FileOrError::E(e) => {
                panic!("{}", e);
            }
        }
        Ok(())
    }
}
impl Default for HackAssembler {
    /// Creates a default `HackAssembler` instance with predefined instructions and labels/symbols.
    ///
    /// This implementation provides default values for the `HackAssembler` struct, including a default
    /// input file name, a default `FileOrError` variant (file not opened), predefined Hack Assembly
    /// instructions, and a set of predefined labels and symbols.
    ///
    /// The predefined symbols are as follows:
    /// - R0-R15: Memory addresses 0 to 15
    /// - SCREEN: Memory address 16384
    /// - KBD: Memory address 24576
    /// - SP: Memory address 0
    /// - LCL: Memory address 1
    /// - ARG: Memory address 2
    /// - THIS: Memory address 3
    /// - THAT: Memory address 4
    ///
    /// The predefined instructions are organized into four categories:
    /// 1. a_zero: Instructions with A-instruction format (@value) where 'value' is numeric.
    /// 2. a_one: Instructions with A-instruction format (@symbol) where 'symbol' is a label or a symbol.
    /// 3. dest: Destination part of C-instruction format (dest=comp;jump).
    /// 4. jump: Jump part of C-instruction format (dest=comp;jump).
    ///
    /// These instructions are stored in a nested HashMap called `instructions`, where the outer HashMap's key
    /// represents the instruction type (e.g., "COMP_A_0" or "COMP_A_1"), and the inner HashMap contains key-value
    /// pairs for each field of the instruction and its corresponding binary representation.
    ///
    /// The method also initializes the `labels_symbols_map` with the predefined labels and symbols as well.
    /// The `labels_symbols_map` is a HashMap that stores the mapping between labels/symbols found in the Hack
    /// Assembly code and their corresponding memory addresses (represented as 32-bit signed integers).
    ///
    /// # Example
    /// ```
    /// // Create a new HackAssembler instance with default values
    /// let assembler = HackAssembler::default();
    /// ```
    fn default() -> Self {
        // Label-Symbol Map
        let predefined_symbols: Vec<(String, i16)> = vec![
            ("R0".to_string(), 0),
            ("R1".to_string(), 1),
            ("R2".to_string(), 2),
            ("R3".to_string(), 3),
            ("R4".to_string(), 4),
            ("R5".to_string(), 5),
            ("R6".to_string(), 6),
            ("R7".to_string(), 7),
            ("R8".to_string(), 8),
            ("R9".to_string(), 9),
            ("R10".to_string(), 10),
            ("R11".to_string(), 11),
            ("R12".to_string(), 12),
            ("R13".to_string(), 13),
            ("R14".to_string(), 14),
            ("R15".to_string(), 15),
            ("SCREEN".to_string(), 16384),
            ("KBD".to_string(), 24576),
            ("SP".to_string(), 0),
            ("LCL".to_string(), 1),
            ("ARG".to_string(), 2),
            ("THIS".to_string(), 3),
            ("THAT".to_string(), 4),
        ];

        // Instruction
        let a_zero: Vec<(String, String)> = vec![
            ("0".to_string(), "101010".to_string()),
            ("1".to_string(), "111111".to_string()),
            ("-1".to_string(), "111010".to_string()),
            ("D".to_string(), "001100".to_string()),
            ("A".to_string(), "110000".to_string()),
            ("!D".to_string(), "001101".to_string()),
            ("!A".to_string(), "110001".to_string()),
            ("-D".to_string(), "001111".to_string()),
            ("-A".to_string(), "110011".to_string()),
            ("D+1".to_string(), "011111".to_string()),
            ("A+1".to_string(), "110111".to_string()),
            ("D-1".to_string(), "001110".to_string()),
            ("A-1".to_string(), "110010".to_string()),
            ("D+A".to_string(), "000010".to_string()),
            ("D-A".to_string(), "010011".to_string()),
            ("A-D".to_string(), "000111".to_string()),
            ("D&A".to_string(), "000000".to_string()),
            ("D|A".to_string(), "010101".to_string()),
        ];
        let a_one: Vec<(String, String)> = vec![
            ("M".to_string(), "110000".to_string()),
            ("!M".to_string(), "110001".to_string()),
            ("-M".to_string(), "110011".to_string()),
            ("M+1".to_string(), "110111".to_string()),
            ("M-1".to_string(), "110010".to_string()),
            ("D+M".to_string(), "000010".to_string()),
            ("D-M".to_string(), "010011".to_string()),
            ("M-D".to_string(), "000111".to_string()),
            ("D&M".to_string(), "000000".to_string()),
            ("D|M".to_string(), "010101".to_string()),
        ];
        let dest: Vec<(String, String)> = vec![
            ("null".to_string(), "000".to_string()),
            ("M".to_string(), "001".to_string()),
            ("D".to_string(), "010".to_string()),
            ("MD".to_string(), "011".to_string()),
            ("A".to_string(), "100".to_string()),
            ("AM".to_string(), "101".to_string()),
            ("AD".to_string(), "110".to_string()),
            ("AMD".to_string(), "111".to_string()),
        ];
        let jump: Vec<(String, String)> = vec![
            ("null".to_string(), "000".to_string()),
            ("JGT".to_string(), "001".to_string()),
            ("JEQ".to_string(), "010".to_string()),
            ("JGE".to_string(), "011".to_string()),
            ("JLT".to_string(), "100".to_string()),
            ("JNE".to_string(), "101".to_string()),
            ("JLE".to_string(), "110".to_string()),
            ("JMP".to_string(), "111".to_string()),
        ];

        fn add_instruction(
            a: Vec<(String, String)>,
            b: Vec<(String, String)>,
            c: Vec<(String, String)>,
            d: Vec<(String, String)>,
        ) -> HashMap<String, HashMap<String, String>> {
            let mut to_return = HashMap::new();
            to_return.insert("COMP_A_0".to_string(), a.into_iter().collect());
            to_return.insert("COMP_A_1".to_string(), b.into_iter().collect());
            to_return.insert("DEST".to_string(), c.into_iter().collect());
            to_return.insert("JUMP".to_string(), d.into_iter().collect());
            to_return
        }

        HackAssembler {
            file_name: String::from("First init.."),
            file: FileOrError::E(String::from("First Init..")),
            instructions: add_instruction(a_zero, a_one, dest, jump),
            labels_symbols_map: predefined_symbols.into_iter().collect(),
        }
    }
}
