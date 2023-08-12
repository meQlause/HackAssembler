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
    fn a_instruction(&self, a: i16) -> String {
        println!("0{:015b} = {}", a, a);
        format!("0{:015b}", a)
    }
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
        println!("{}{}{}{} = {:?}", to_return, comp, dst, jmp, ins.join("="));
        to_return + &comp + &dst + &jmp
    }

    fn write(&mut self, text: &str) {
        writeln!(self.file, "{}", text).expect("can't write.");
    }
}
