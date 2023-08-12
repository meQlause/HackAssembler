mod code;
mod parser;
mod table;
use code::{Code, CodeTrait};
use parser::{ParseTrait, Parser};
use std::env;
use table::Table;
fn main() {
    let args: Vec<String> = env::args().collect();

    let mut par = Parser::new(&args[1].to_string());
    let mut table = Table::new();
    let mut code: Code = Code::new(args[2].to_string());
    table.init_label(&mut par);
    loop {
        if par.set_command() {
            if let Some(c) = par.current_command.chars().next() {
                if c == '@' {
                    let value: i16 = if let Some(val) =
                        table.tables.get(&par.current_command[1..].to_string())
                    {
                        *val
                    } else {
                        let val = par.current_command.clone().trim()[1..].to_string();
                        let to_return = val.parse::<i16>().unwrap();
                        to_return
                    };

                    code.write(&code.a_instruction(value));
                } else if c != '/' && c != ' ' && c != '(' {
                    code.write(&code.c_instruction(&par.current_command, &table));
                }
            }
            continue;
        }
        break;
    }
}
