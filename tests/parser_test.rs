extern crate whitespace;

use std::fs::File;
use std::io::prelude::*;
use whitespace::parser::*;

#[test]
fn it_can_run_count() {
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




use proptest::prelude::*;
proptest! {
    #[test]
    fn it_parses_any_int_without_crashes(s in ".*") {
        let _ = match_int(&s);
    }

    #[test]
    fn it_parses_any_regular_int_without_crashes(s in "[ \t\n]*") {
        let _ = match_int(&s);
    }

    #[test]
    fn it_parses_any_string_without_crashes(s in ".*") {
        let _ = match_string(&s);
    }

    #[test]
    fn it_parses_any_regular_string_without_crashes(s in "[ \t\n]*") {
        let _ = match_string(&s);
    }

    // #[test]
    // fn it_parses_anything_without_crashes(s in ".*") {
    //     let _ = parse_program(&s);
    // }
}