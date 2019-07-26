use crate::instruction::*;

type ParseResult<'a, Output> = Result<(&'a str, Output), String>;



// low values are on the right
pub fn match_int(input: &str) -> ParseResult<i32> {
    let mut int_value: i32 = 0;
    let mut offset = 0;

    for c in input.chars() {
        match c {
            '\t' => {
                int_value = (int_value << 1) + 1;
                offset += 1;
            }
            ' ' => {
                int_value <<= 1;
                offset += 1;
            }
            '\n' => {
                return if offset > 0 {
                    Ok((&input[(offset + 1)..], int_value))
                } else {
                    Err("The int terminates with no data".to_string())
                }
            }
            c => {
                return Err(format!(
                    "{} is not a valid character when trying to match an int",
                    c
                ))
            }
        }
    }

    Err("an int should terminate with a \\n".to_string())
}

pub fn match_string(input: &str) -> ParseResult<String> {
    let mut matched = String::new();

    let mut curr_byte: u8 = 0u8;
    let mut offset: usize = 0;

    for c in input.chars() {
        match c {
            '\t' => {
                curr_byte = (curr_byte << 1) + 1;
                offset += 1;
            }
            ' ' => {
                curr_byte <<= 1;
                offset += 1;
            }
            '\n' => {
                if offset % 8 != 0 {
                    matched += &(curr_byte as char).to_string();
                }
                return if !matched.is_empty() {
                    Ok((&input[(offset + 1)..], matched))
                } else {
                    Err("The string terminates with no data".to_string())
                };
            }
            c => {
                return Err(format!(
                    "{} is not a valid character when trying to match a string",
                    c
                ))
            }
        }

        if offset % 8 == 0 {
            matched += &(curr_byte as char).to_string();
            curr_byte = 0;
        }
    }

    Err("a string should terminate with a \\n".to_string())
}

pub fn debug(ws_program: &str) {
    for c in ws_program.chars() {
        match c {
            '\t' => print!("[tab]"),
            ' ' => print!("[space]"),
            _ => print!("{}", c),
        }
    }
}

fn create_push_instruction(x: (&str, i32)) -> ParseResult<Instruction> {
    Ok((x.0, Instruction::Push(x.1)))
}

fn create_copynth_instruction(x: (&str, i32)) -> ParseResult<Instruction> {
    Ok((x.0, Instruction::CopyNth(x.1)))
}

fn create_slide_instruction(x: (&str, i32)) -> ParseResult<Instruction> {
    Ok((x.0, Instruction::Slide(x.1)))
}

fn create_setlabel_instruction(x: (&str, String)) -> ParseResult<Instruction> {
    Ok((x.0, Instruction::SetLabel(x.1)))
}

fn create_callsubroutine_instruction(x: (&str, String)) -> ParseResult<Instruction> {
    Ok((x.0, Instruction::CallSubroutine(x.1)))
}

fn create_jump_instruction(x: (&str, String)) -> ParseResult<Instruction> {
    Ok((x.0, Instruction::Jump(x.1)))
}

fn create_jzero_instruction(x: (&str, String)) -> ParseResult<Instruction> {
    Ok((x.0, Instruction::JZero(x.1)))
}

fn create_jneg_instruction(x: (&str, String)) -> ParseResult<Instruction> {
    Ok((x.0, Instruction::JNeg(x.1)))
}

fn parse_start_space<'a >(chars: &Vec<char>, ws_program: &'a str) -> ParseResult<'a, Instruction> {
    if chars[1] == ' ' {
        match_int(&ws_program[2..]).and_then(create_push_instruction)
    } else if chars[1] == '\n' {
        if chars[2] == ' ' {
            Ok((&ws_program[3..], Instruction::Duplicate))
        } else if chars[2] == '\t' {
            Ok((&ws_program[3..], Instruction::Swap))
        } else if chars[2] == '\n' {
            Ok((&ws_program[3..], Instruction::Discard))
        } else {
            Err(format!("{} is an unexpected character", chars[2]))
        }
    } else if chars[1] == '\t' {
        if chars[2] == ' ' {
            match_int(&ws_program[3..]).and_then(create_copynth_instruction)
        } else if chars[2] == '\n' {
            match_int(&ws_program[3..]).and_then(create_slide_instruction)
        } else {
            Err(format!("{} is an unexpected character", chars[2]))
        }
    } else {
        Err(format!("{} is an unexpected character", chars[1]))
    }
}

fn parse_instruction(ws_program: &str) -> ParseResult<Instruction> {
    let chars: Vec<char> = ws_program.chars().collect();
    match chars[0] {
        ' ' => parse_start_space(&chars, &ws_program),
        '\t' => {
            if chars[1] == ' ' {
                if chars[2] == ' ' {
                    if chars[3] == ' ' {
                        Ok((&ws_program[4..], Instruction::Add))
                    } else if chars[3] == '\t' {
                        Ok((&ws_program[4..], Instruction::Sub))
                    } else if chars[3] == '\n' {
                        Ok((&ws_program[4..], Instruction::Mul))
                    } else {
                        Err(format!("{} is an unexpected character", chars[3]))
                    }
                } else if chars[2] == '\t' {
                    if chars[3] == ' ' {
                        Ok((&ws_program[4..], Instruction::Div))
                    } else if chars[3] == '\t' {
                        Ok((&ws_program[4..], Instruction::Mod))
                    } else {
                        Err(format!("{} is an unexpected character", chars[3]))
                    }
                } else {
                    Err(format!("{} is an unexpected character", chars[2]))
                }
            } else if chars[1] == '\t' {
                if chars[2] == ' ' {
                    Ok((&ws_program[3..], Instruction::Store))
                } else if chars[2] == '\t' {
                    Ok((&ws_program[3..], Instruction::Retrieve))
                } else {
                    Err(format!("{} is an unexpected character", chars[2]))
                }
            } else if chars[1] == '\n' {
                if chars[2] == ' ' {
                    if chars[3] == ' ' {
                        Ok((&ws_program[4..], Instruction::PrintChar))
                    } else if chars[3] == '\t' {
                        Ok((&ws_program[4..], Instruction::PrintInt))
                    } else {
                        Err(format!("{} is an unexpected character", chars[3]))
                    }
                } else if chars[2] == '\t' {
                    if chars[3] == ' ' {
                        Ok((&ws_program[4..], Instruction::ReadChar))
                    } else if chars[3] == '\t' {
                        Ok((&ws_program[4..], Instruction::ReadInt))
                    } else {
                        Err(format!("{} is an unexpected character", chars[3]))
                    }
                } else {
                    Err(format!("{} is an unexpected character", chars[2]))
                }
            } else {
                Err(format!("{} is an unexpected character", chars[1]))
            }
        }
        '\n' => {
            if chars[1] == ' ' {
                if chars[2] == ' ' {
                    match_string(&ws_program[3..]).and_then(create_setlabel_instruction)
                } else if chars[2] == '\t' {
                    match_string(&ws_program[3..]).and_then(create_callsubroutine_instruction)
                } else if chars[2] == '\n' {
                    match_string(&ws_program[3..]).and_then(create_jump_instruction)
                } else {
                    Err(format!("{} is an unexpected character", chars[2]))
                }
            } else if chars[1] == '\n' {
                if chars[2] == ' ' || chars[2] == '\t' {
                    Err(format!("{} is an unexpected character", chars[2]))
                } else if chars[2] == '\n' {
                    Ok((&ws_program[3..], Instruction::EndOfProgram))
                } else {
                    Err(format!("{} is an unexpected character", chars[2]))
                }
            } else if chars[1] == '\t' {
                if chars[2] == ' ' {
                    match_string(&ws_program[3..]).and_then(create_jzero_instruction)
                } else if chars[2] == '\t' {
                    match_string(&ws_program[3..]).and_then(create_jneg_instruction)
                } else if chars[2] == '\n' {
                    Ok((&ws_program[3..], Instruction::EndOfSubroutine))
                } else {
                    Err(format!("{} is an unexpected character", chars[2]))
                }
            } else {
                Err(format!("{} is an unexpected character", chars[1]))
            }
        }
        _ => Err(format!("{} is an unexpected character", chars[0])),
    }
}

#[derive(PartialEq, Debug)]
pub enum Token {
    Whitspace,
    Tab,
    LineFeed
}

pub fn tokenize(ws_program: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    ws_program
        .chars()
        .for_each(|c| {
            match c {
                ' ' =>  tokens.push(Token::Whitspace),
                '\t' => tokens.push(Token::Tab),
                '\n' => tokens.push(Token::LineFeed),
                _ => {} 
            }
        });
    tokens
}

pub fn parse_program(ws_program: &str) -> Result<Vec<Instruction>, &str> {
    let mut ws_program: &str = &(ws_program
        .chars()
        .filter(|c| *c == ' ' || *c == '\t' || *c == '\n')
        .collect::<String>());
    let mut instructions: Vec<Instruction> = Vec::new();
    while ws_program != "" {
        let result = parse_instruction(ws_program);
        match result {
            Ok((rest_of_program, instruction)) => {
                ws_program = rest_of_program;
                instructions.push(instruction);
            }
            Err(_) => return Err("unable to parse the current instruction"),
        }
    }
    Ok(instructions)
}




#[test]
fn tokenize_works() {
    use Token::*;
    assert_eq!(vec![Whitspace, LineFeed, Tab], tokenize("pouet \n\tabc"));
}




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