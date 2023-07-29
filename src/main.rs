mod assembler;
use assembler::HackAssembler;
use std::env;
fn main() {
    // Get the command-line arguments as a collection of strings
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        // If the user doesn't provide 2 arguments (input file and output file),
        // print an explanation of how to use the program
        println!("Usage: {} <input_file> <output_file>", args[0]);
        println!("Please provide the input file and output file names.");
    } else {
        // If 2 arguments are provided, proceed with assembling the code
        let mut assembler = HackAssembler::new(&args[1].to_string());
        assembler.load_file();
        match assembler.assemble(&args[2].to_string()) {
            Ok(_) => {
                println!("Conversion completed.");
            }
            Err(error_msg) => {
                println!("Error: {}", error_msg);
            }
        }
    }
}
