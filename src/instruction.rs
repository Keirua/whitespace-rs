#[derive(PartialEq, Clone)]
pub enum Instruction {
    // stack
    Push(i32),
    Duplicate,
    CopyNth(i32),
    Swap,
    Discard,
    Slide(i32),
    // arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    // heap
    Store,
    Retrieve,
    // flow control
    SetLabel(String),
    CallSubroutine(String),
    Jump(String),
    JZero(String),
    JNeg(String),
    EndOfSubroutine,
    EndOfProgram,
    // I/O
    PrintChar,
    PrintInt,
    ReadChar,
    ReadInt,
}

use std::fmt;
impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Push(v) => write!(f, "Push({})", v),
            Instruction::Duplicate => write!(f, "Duplicate"),
            Instruction::CopyNth(v) => write!(f, "Push({})", v),
            Instruction::Swap => write!(f, "Swap"),
            Instruction::Discard => write!(f, "Discard"),
            Instruction::Slide(v) => write!(f, "Slide({})", v),
            Instruction::Add => write!(f, "Add"),
            Instruction::Sub => write!(f, "Sub"),
            Instruction::Mul => write!(f, "Mul"),
            Instruction::Div => write!(f, "Div"),
            Instruction::Mod => write!(f, "Mod"),
            Instruction::Store => write!(f, "Store"),
            Instruction::Retrieve => write!(f, "Retrieve"),
            Instruction::SetLabel(label) => write!(f, "SetLabel(\"{}\")", label),
            Instruction::CallSubroutine(subroutine) => {
                write!(f, "CallSubroutine(\"{}\")", subroutine)
            }
            Instruction::Jump(label) => write!(f, "Jump(\"{}\")", label),
            Instruction::JZero(label) => write!(f, "JZero(\"{}\")", label),
            Instruction::JNeg(label) => write!(f, "JNeg(\"{}\")", label),
            Instruction::EndOfSubroutine => write!(f, "EndOfSubroutine"),
            Instruction::EndOfProgram => write!(f, "EndOfProgram"),
            Instruction::PrintChar => write!(f, "PrintChar"),
            Instruction::PrintInt => write!(f, "PrintInt"),
            Instruction::ReadChar => write!(f, "ReadChar"),
            Instruction::ReadInt => write!(f, "ReadInt"),
        }
    }
}

