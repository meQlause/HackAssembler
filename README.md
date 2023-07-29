# HackAssembler
The Hack Assembler is a simple Hack Assembly language assembler that allows you to load Hack Assembly code from a file, process the instructions, and generate corresponding machine code. The assembler supports both built-in instructions and user-defined labels/symbols for memory addresses. The machine code is represented as binary strings.

# Getting Started
To use the Hack Assembler, follow these steps:

1. Clone or download this repository to your local machine.
2. Ensure you have Rust installed on your system. If not, you can download it from the official website: https://www.rust-lang.org/tools/install
3. Open a terminal or command prompt and navigate to the directory where you cloned/downloaded the repository.
4. Use the following commands to build and run the assembler:

```console
cargo build
cargo run -- <input_file> <output_file>
```
Replace <input_file> with the path to your Hack Assembly code file (e.g., program.asm).
Replace <output_file> with the path to your converted Hack Assembly code file (e.g., program.hack).

The assembled machine code will be written to a file named <outpur_file>.

# Code Overview
## 'FileOrError' Enum
The **'FileOrError'** enum represents either an open file handle (**File**) or an error message (**String**). It is used in the context of the HackAssembler struct to handle the input file. It allows capturing both the successfully opened file and any potential errors that occurred during file operations.

# HackAssembler Struct
The **HackAssembler** struct represents the assembler itself. It contains the following fields:

1. **file_name**: The name of the input file containing the Hack Assembly code.
2. **file**: A file handle that represents the opened input file or an error if the file could not be opened or read.
3. **instructions**: A nested HashMap representing the Hack Assembly instructions. The outer HashMap's key is the mnemonic (e.g., "A-M" or "0;JMP"), and the inner HashMap contains key-value pairs for each field (e.g., "dest", "comp", "jump") of the instruction and its corresponding binary representation.
4. **labels_symbols_map**: A HashMap that stores the mapping between labels/symbols found in the Hack Assembly code and their memory addresses (represented as i16-bit signed integers).

# Methods
1. **load_file(&mut self, file: &String)**: Loads an input file containing Hack Assembly code into the HackAssembler. This method updates the file_name field and attempts to open the specified file. If the file is successfully opened, it adds labels and symbols to the labels_symbols_map.
2. **assemble(&self, file: &String) -> Result<()>**: Assembles the loaded Hack Assembly code and writes the resulting machine code to the specified file. The machine code is represented as binary strings.
3. **a_instruction(&self, a: &i16) -> String**: Converts an A-instruction value (numeric) to its binary representation.
4. **c_instruction(&self, a: &String) -> String**: Converts a C-instruction to its binary representation.
5. **add_labels_symbols_map(&mut self)**: Internal method to add labels and symbols to the labels_symbols_map. This method is called during file loading to build the mapping between labels/symbols and memory addresses.
