extern crate whitespace;

use std::fs::File;
use std::io::prelude::*;
use whitespace::instruction::*;
use whitespace::parser::*;

#[test]
fn match_integer_works() {
    assert_eq!(
        Err("a is not a valid character when trying to match an int".to_string()),
        match_int("abc")
    );
    assert_eq!(
        Err("an int should terminate with a \\n".to_string()),
        match_int("")
    );
    assert_eq!(
        Err("The int terminates with no data".to_string()),
        match_int("\n")
    );
    assert_eq!(
        Err("an int should terminate with a \\n".to_string()),
        match_int(" ")
    );
    assert_eq!(
        Err("an int should terminate with a \\n".to_string()),
        match_int("\t")
    );
    assert_eq!(Ok(("", 0)), match_int(" \n"));
    assert_eq!(Ok(("", 1)), match_int("\t\n"));
    assert_eq!(Ok(("abc", 1)), match_int("\t\nabc"));
    assert_eq!(Ok(("", 1)), match_int(" \t\n"));
    assert_eq!(Ok(("", 2)), match_int("\t \n"));
    assert_eq!(Ok(("", 4)), match_int("\t  \n"));
    assert_eq!(Ok(("", 8)), match_int("\t   \n"));
    assert_eq!(Ok(("", 8)), match_int(" \t   \n"));
    assert_eq!(Ok(("", 3)), match_int("\t\t\n"));
    assert_eq!(Ok(("", 27)), match_int("\t\t \t\t\n"));
    assert_eq!(Ok(("abc", 27)), match_int("\t\t \t\t\nabc"));
    assert_eq!(
        Err("an int should terminate with a \\n".to_string()),
        match_int("\t\t \t\t")
    );
}

#[test]
fn match_string_works() {
    assert_eq!(
        Err("a is not a valid character when trying to match a string".to_string()),
        match_string("abc")
    );
    assert_eq!(
        Err("a string should terminate with a \\n".to_string()),
        match_string("")
    );
    assert_eq!(
        Err("The string terminates with no data".to_string()),
        match_string("\n")
    );
    assert_eq!(
        Err("a string should terminate with a \\n".to_string()),
        match_string(" ")
    );
    assert_eq!(
        Err("a string should terminate with a \\n".to_string()),
        match_string("\t")
    );
    assert_eq!(
        Ok(("", vec![0 as char].into_iter().collect::<String>())),
        match_string(" \n")
    );
    assert_eq!(
        Ok(("", vec![1 as char].into_iter().collect::<String>())),
        match_string("\t\n")
    );
    assert_eq!(
        Ok(("abc", vec![1 as char].into_iter().collect::<String>())),
        match_string("\t\nabc")
    );
    assert_eq!(
        Ok(("", vec![1 as char].into_iter().collect::<String>())),
        match_string(" \t\n")
    );
    assert_eq!(
        Ok(("", vec![8 as char].into_iter().collect::<String>())),
        match_string(" \t   \n")
    );
    assert_eq!(
        Ok(("", vec![3 as char].into_iter().collect::<String>())),
        match_string("\t\t\n")
    );
    assert_eq!(
        Ok(("", vec![27 as char].into_iter().collect::<String>())),
        match_string("\t\t \t\t\n")
    );
    assert_eq!(
        Ok(("abc", vec![27 as char].into_iter().collect::<String>())),
        match_string("\t\t \t\t\nabc")
    );
    assert_eq!(Ok(("abc", "A".to_string())), match_string("\t     \t\nabc"));
    assert_eq!(
        Ok(("abc", "a".to_string())),
        match_string("\t\t    \t\nabc")
    );
    assert_eq!(
        Ok((
            "",
            vec![3 as char, 1 as char].into_iter().collect::<String>()
        )),
        match_string("      \t\t\t\n")
    );
    assert_eq!(
        Ok((
            "",
            vec![3 as char, 0 as char].into_iter().collect::<String>()
        )),
        match_string("      \t\t \n")
    );
    assert_eq!(
        Ok((
            "",
            vec![3 as char, 3 as char].into_iter().collect::<String>()
        )),
        match_string("      \t\t      \t\t\n")
    );
    assert_eq!(
        Err("a string should terminate with a \\n".to_string()),
        match_string("\t\t \t\t")
    );
}

#[test]
fn parse_instruction_works() {
    // Stack
    assert_eq!(
        Err("a is an unexpected character".to_string()),
        parse_instruction("abc")
    );
    assert_eq!(
        Ok(("abc", Instruction::Push(1))),
        parse_instruction("  \t\nabc")
    );
    assert_eq!(
        Ok(("abc", Instruction::Push(2))),
        parse_instruction("  \t \nabc")
    );
    assert_eq!(
        Ok(("abc", Instruction::Push(2))),
        parse_instruction("   \t \nabc")
    );
    assert_eq!(
        Ok(("abc", Instruction::Push(5))),
        parse_instruction("  \t \t\nabc")
    );

    assert_eq!(
        Ok(("abc", Instruction::Duplicate)),
        parse_instruction(" \n abc")
    );
    assert_eq!(
        Ok(("abc", Instruction::CopyNth(3))),
        parse_instruction(" \t \t\t\nabc")
    );
    assert_eq!(
        Ok(("abc", Instruction::Swap)),
        parse_instruction(" \n\tabc")
    );
    assert_eq!(
        Ok(("abc", Instruction::Discard)),
        parse_instruction(" \n\nabc")
    );
    assert_eq!(
        Ok(("abc", Instruction::Slide(3))),
        parse_instruction(" \t\n\t\t\nabc")
    );

    // Arithmetic
    assert_eq!(Ok(("abc", Instruction::Add)), parse_instruction("\t   abc"));
    assert_eq!(
        Ok(("abc", Instruction::Sub)),
        parse_instruction("\t  \tabc")
    );
    assert_eq!(
        Ok(("abc", Instruction::Mul)),
        parse_instruction("\t  \nabc")
    );
    assert_eq!(
        Ok(("abc", Instruction::Div)),
        parse_instruction("\t \t abc")
    );
    assert_eq!(
        Ok(("abc", Instruction::Mod)),
        parse_instruction("\t \t\tabc")
    );

    // Heap
    assert_eq!(
        Ok(("abc", Instruction::Store)),
        parse_instruction("\t\t abc")
    );
    assert_eq!(
        Ok(("abc", Instruction::Retrieve)),
        parse_instruction("\t\t\tabc")
    );

    // Flow
    // …many instructions missing
    assert_eq!(
        Ok(("abc", Instruction::SetLabel("A".to_string()))),
        parse_instruction("\n  \t     \t\nabc")
    );
    assert_eq!(
        Ok(("abc", Instruction::CallSubroutine("A".to_string()))),
        parse_instruction("\n \t\t     \t\nabc")
    );
    assert_eq!(
        Ok(("abc", Instruction::Jump("A".to_string()))),
        parse_instruction("\n \n\t     \t\nabc")
    );
    assert_eq!(
        Ok(("abc", Instruction::JZero("A".to_string()))),
        parse_instruction("\n\t \t     \t\nabc")
    );
    assert_eq!(
        Ok(("abc", Instruction::JNeg("A".to_string()))),
        parse_instruction("\n\t\t\t     \t\nabc")
    );

    assert_eq!(
        Ok(("abc", Instruction::EndOfSubroutine)),
        parse_instruction("\n\t\nabc")
    );
    assert_eq!(
        Ok(("abc", Instruction::EndOfProgram)),
        parse_instruction("\n\n\nabc")
    );

    // I/O
    assert_eq!(
        Ok(("abc", Instruction::PrintChar)),
        parse_instruction("\t\n  abc")
    );
    assert_eq!(
        Ok(("abc", Instruction::PrintInt)),
        parse_instruction("\t\n \tabc")
    );
    assert_eq!(
        Ok(("abc", Instruction::ReadChar)),
        parse_instruction("\t\n\t abc")
    );
    assert_eq!(
        Ok(("abc", Instruction::ReadInt)),
        parse_instruction("\t\n\t\tabc")
    );
}

#[test]
fn parse_program_works() {
    // Stack
    assert_eq!(Ok(vec![Instruction::Push(1)]), parse_program("  \t\n"));
    assert_eq!(
        Ok(vec![Instruction::Push(1), Instruction::Duplicate]),
        parse_program("  \t\n \n ")
    );
}

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
