mod assembler;
mod code;
mod parser;
mod table;
use assembler::HackAssembler;
use code::Code;
use parser::Parser;
use std::env;
use table::Table;
fn main() {
    //     // Get the command-line arguments as a collection of strings
    //     let args: Vec<String> = env::args().collect();

    //     if args.len() != 3 {
    //         // If the user doesn't provide 2 arguments (input file and output file),
    //         // print an explanation of how to use the program
    //         println!("Usage: {} <input_file> <output_file>", args[0]);
    //         println!("Please provide the input file and output file names.");
    //     } else {
    //         // If 2 arguments are provided, proceed with assembling the code
    //         let mut assembler = HackAssembler::new(&args[1].to_string());
    //         assembler.load_file();
    //         match assembler.assemble(&args[2].to_string()) {
    //             Ok(_) => {
    //                 println!("Conversion completed.");
    //             }
    //             Err(error_msg) => {
    //                 println!("Error: {}", error_msg);
    //             }
    //         }
    //     }

    let mut par = Parser::new("test.asm");
    let mut table = Table::new();
    let mut code: Code = Code::new("test.hack".to_string());
    table.init_label(&mut par);
    // println!("{:?}", table.tables);
    loop {
        if par.set_command() {
            if par.current_command.chars().next() == Some('@') {
                if let Some(val) = table.tables.get(&par.current_command.to_string()) {
                    code.write(&code.c_instruction(&par.current_command.to_string(), &table));
                } else {
                    code.write(
                        &code.a_instruction(&par.current_command[1..].parse::<i16>().unwrap()),
                    )
                }
            }
        }
        break;
    }
}
