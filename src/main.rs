mod assembler;
use assembler::HackAssembler;
fn main() {
    let mut ardial = HackAssembler::default();
    // ardial.display();
    ardial.load_file(&"test.txt".to_string());
    // ardial.display();
    match ardial.assemble(&"test.hack".to_string()) {
        Ok(_) => {
            println!("Converted.");
        }
        Err(error_msg) => {
            println!("Error: {}", error_msg);
        }
    }
}
