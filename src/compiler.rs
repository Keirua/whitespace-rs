use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use crate::instruction::*;

pub fn generate_source_code(instructions: &Vec<Instruction>, path: &PathBuf) {
    let display = path.display();
    let mut file = match File::create(path) {
        Err(why) => panic!("couldn't create {}: {:?}", display, why),
        Ok(file) => file,
    };

    let beginning = String::from("extern crate whitespace;

use whitespace::parser::*;
use whitespace::vm::*;
use whitespace::instruction::*;

fn main() -> std::io::Result<()> {
    use Instruction::*;
    let instructions = vec![
");

    let end = String::from("
    ];

    let mut vm = VirtualMachine::new(&instructions);

    vm.run_program();
    Ok(())
}
");
    let _ = file.write(&beginning.as_bytes());
    for instruction in instructions.iter() {
        let _ = file.write("\t\t".as_bytes());
        let _ = write_as_rust_string(&instruction, &mut file);
        let _ = file.write(",\n".as_bytes());
    }

    println!("{:?}", instructions);

    let _ = file.write(&end.as_bytes());
}


pub fn write_as_rust_string(instruction: &Instruction, file: &mut File) {
    let _ = match instruction {
        Instruction::Push(v) => file.write(format!("Push({})", v).as_bytes()),
        Instruction::Duplicate => file.write("Duplicate".as_bytes()),
        Instruction::CopyNth(v) => file.write(format!("Push({})", v).as_bytes()),
        Instruction::Swap => file.write("Swap".as_bytes()),
        Instruction::Discard => file.write("Discard".as_bytes()),
        Instruction::Slide(v) => file.write(format!("Slide({})", v).as_bytes()),
        Instruction::Add => file.write("Add".as_bytes()),
        Instruction::Sub => file.write("Sub".as_bytes()),
        Instruction::Mul => file.write("Mul".as_bytes()),
        Instruction::Div => file.write("Div".as_bytes()),
        Instruction::Mod => file.write("Mod".as_bytes()),
        Instruction::Store => file.write("Store".as_bytes()),
        Instruction::Retrieve => file.write("Retrieve".as_bytes()),
        Instruction::SetLabel(label) => file.write(format!("SetLabel(\"{}\".to_string())", label).as_bytes()),
        Instruction::CallSubroutine(subroutine) => 
            file.write(format!("CallSubroutine(\"{}\".to_string())", subroutine).as_bytes()),
        
        Instruction::Jump(label) => file.write(format!("Jump(\"{}\".to_string())", label).as_bytes()),
        Instruction::JZero(label) => file.write(format!("JZero(\"{}\".to_string())", label).as_bytes()),
        Instruction::JNeg(label) => file.write(format!("JNeg(\"{}\".to_string())", label).as_bytes()),
        Instruction::EndOfSubroutine => file.write("EndOfSubroutine".as_bytes()),
        Instruction::EndOfProgram => file.write("EndOfProgram".as_bytes()),
        Instruction::PrintChar => file.write("PrintChar".as_bytes()),
        Instruction::PrintInt => file.write("PrintInt".as_bytes()),
        Instruction::ReadChar => file.write("ReadChar".as_bytes()),
        Instruction::ReadInt => file.write("ReadInt".as_bytes()),
    };
}