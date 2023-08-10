use std::collections::HashMap;

struct Table {
    instructions: HashMap<String, HashMap<String, String>>,
    tables: HashMap<String, i16>,
}

impl Table {
    pub fn new() -> Table {
        let predefined_table: Vec<(String, i16)> = vec![
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

        Table {
            instructions: add_instruction(a_zero, a_one, dest, jump),
            tables: predefined_table.into_iter().collect(),
        }
    }

    fn get_a0(&self, key: &str) -> String {
        let t = self.instructions.get("COMP_A_0").unwrap();
        t.get(key).unwrap().to_string()
    }
    fn get_a1(&self, key: &str) -> String {
        let t = self.instructions.get("COMP_A_1").unwrap();
        t.get(key).unwrap().to_string()
    }
    fn get_dst(&self, key: &str) -> String {
        let t = self.instructions.get("DEST").unwrap();
        t.get(key).unwrap().to_string()
    }
    fn get_jmp(&self, key: &str) -> String {
        let t = self.instructions.get("JUMP").unwrap();
        t.get(key).unwrap().to_string()
    }
    fn add(&mut self, key: &str, num: i16) {
        self.tables.insert(key.to_string(), num);
    }
}
