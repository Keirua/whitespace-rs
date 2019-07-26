extern crate whitespace;

use std::fs::File;
use std::io::prelude::*;
use whitespace::parser::*;
use whitespace::*;

#[test]
fn instruction_push_works() {
    use whitespace::instruction::Instruction::*;
    let expected_tokens = vec![
        Push(1),
        SetLabel("C".to_string()),
        Duplicate,
        PrintInt,
        Push(10),
        PrintChar,
        Push(1),
        Add,
        Duplicate,
        Push(11),
        Sub,
        JZero("E".to_string()),
        Jump("C".to_string()),
        SetLabel("E".to_string()),
        Discard,
        EndOfProgram,
    ];

    let mut file = File::open("examples/count2.ws").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let tokens = parse_program(&contents);

    assert_eq!(tokens, Ok(expected_tokens));
}
