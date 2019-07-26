use crate::instruction::*;
use std::collections::HashMap;
type ExecutionResult<'a> = Result<(), &'a str>;

pub struct VirtualMachine {
    pub stack: Vec<i32>,
    pub heap: Vec<i32>,
    pub call_stack: Vec<usize>,
    pub ip: usize,
    pub labels: HashMap<String, usize>,
    pub instructions: Vec<Instruction>,
    pub debug: bool,
}

impl VirtualMachine {
    pub fn new(instructions: &[Instruction]) -> VirtualMachine {
        let mut vm = VirtualMachine {
            stack: Vec::with_capacity(64),
            call_stack: Vec::with_capacity(64),
            heap: Vec::new(),
            ip: 0,
            labels: HashMap::new(),
            instructions: instructions.to_vec(),
            debug: false,
        };

        for (index, instruction) in instructions.iter().enumerate() {
            if let Instruction::SetLabel(label) = instruction {
                vm.labels.insert(label.to_string(), index);
            }
        }

        vm
    }

    // Run n instructions and returns the last execution
    pub fn run_n(&mut self, n: usize) -> ExecutionResult {
        for _ in 0..(n - 1) {
            if self.debug {
                println!("{:?}", self.instructions[self.ip]);
                println!("\t{:?}", self.stack);
                println!("\t{:?}", self.ip);
            }
            let _ = self.run_instruction();
            if self.debug {
                println!("\t{:?}", self.stack);
                println!("\t{:?}", self.ip);
            }
        }
        if self.debug {
            println!("{:?}", self.instructions[self.ip]);
            println!("\t{:?}", self.stack);
            println!("\t{:?}", self.ip);
        }
        self.run_instruction()
    }

    pub fn run_program(&mut self) {
        println!("Labels:");
        println!("{:?}", self.labels);

        while self.instructions[self.ip] != Instruction::EndOfProgram {
            if self.debug {
                println!("{:?}", self.instructions[self.ip]);
                println!("\t{:?}", self.stack);
                println!("\t{:?}", self.ip);
            }
            if let Err(ref e) = self.run_instruction() {
                println!("{:?}", e);
                break;
            }
            if self.debug {
                println!("\t{:?}", self.stack);
                println!("\t{:?}", self.ip);
            }
        }
    }

    pub fn run_instruction(&mut self) -> ExecutionResult {
        let instruction = self.instructions[self.ip].clone();
        match instruction {
            Instruction::Push(v) => self.run_push(v),
            Instruction::Duplicate => self.run_duplicate(),
            Instruction::CopyNth(v) => self.run_copynth(v),
            Instruction::Swap => self.run_swap(),
            Instruction::Discard => self.run_discard(),
            Instruction::Slide(v) => self.run_slide(v),
            Instruction::Add => self.run_add(),
            Instruction::Sub => self.run_sub(),
            Instruction::Mul => self.run_mul(),
            Instruction::Div => self.run_div(),
            Instruction::Mod => self.run_mod(),
            Instruction::Store => self.run_store(),
            Instruction::Retrieve => self.run_retrieve(),
            Instruction::SetLabel(_) => self.move_on(),
            Instruction::CallSubroutine(subroutine) => self.run_callsubroutine(&subroutine.clone()),
            Instruction::Jump(label) => self.run_jump(&label.clone()),
            Instruction::JZero(label) => self.run_jzero(&label.clone()),
            Instruction::JNeg(label) => self.run_jneg(&label.clone()),
            Instruction::EndOfSubroutine => self.run_endofsubroutine(),
            Instruction::EndOfProgram => Ok(()),
            Instruction::PrintChar => self.run_printchar(),
            Instruction::PrintInt => self.run_printint(),
            Instruction::ReadChar => self.run_readchar(),
            Instruction::ReadInt => self.run_readint(),
        }
    }

    fn move_on(&mut self) -> ExecutionResult {
        self.ip += 1;
        Ok(())
    }

    fn run_push(&mut self, v: i32) -> ExecutionResult {
        self.stack.push(v);
        self.move_on()
    }

    fn run_duplicate(&mut self) -> ExecutionResult {
        if !self.stack.is_empty() {
            self.stack.push(self.stack[self.stack.len() - 1]);
            self.move_on()
        } else {
            Err("Can't duplicate: stack is empty")
        }
    }

    fn run_copynth(&mut self, _v: i32) -> ExecutionResult {
        unimplemented!("Todo !")
    }
    fn run_swap(&mut self) -> ExecutionResult {
        let len = self.stack.len();
        if len >= 2 {
            self.stack.swap(len - 1, len - 2);
            self.move_on()
        } else {
            Err("Can't swap: not enough elements on the stack")
        }
    }
    fn run_discard(&mut self) -> ExecutionResult {
        if !self.stack.is_empty() {
            let _ = self.stack.pop();
            self.move_on()
        } else {
            Err("Can't discard: stack is empty")
        }
    }
    fn run_slide(&mut self, _v: i32) -> ExecutionResult {
        unimplemented!("Todo !")
    }

    fn run_stack_instruction_2_params(&mut self, f: impl Fn(i32, i32) -> i32) -> ExecutionResult {
        let len = self.stack.len();
        if len >= 2 {
            let right = self.stack.pop().unwrap();
            let left = self.stack.pop().unwrap();
            self.stack.push(f(left, right));
            self.move_on()
        } else {
            Err("Can't perform operation: not enough elements on the stack")
        }
    }

    fn run_add(&mut self) -> ExecutionResult {
        self.run_stack_instruction_2_params(|l, r| l + r)
    }
    fn run_sub(&mut self) -> ExecutionResult {
        self.run_stack_instruction_2_params(|l, r| l - r)
    }
    fn run_mul(&mut self) -> ExecutionResult {
        self.run_stack_instruction_2_params(|l, r| l * r)
    }
    fn run_div(&mut self) -> ExecutionResult {
        self.run_stack_instruction_2_params(|l, r| l / r)
    }
    fn run_mod(&mut self) -> ExecutionResult {
        self.run_stack_instruction_2_params(|l, r| l % r)
    }
    fn run_store(&mut self) -> ExecutionResult {
        let len = self.stack.len();
        if len >= 2 {
            let value = self.stack.pop().unwrap();
            let address = self.stack.pop().unwrap();

            self.write_heap(address, value);
            self.move_on()
        } else {
            Err("Can't perform operation: not enough elements on the stack")
        }
    }
    fn run_retrieve(&mut self) -> ExecutionResult {
        if !self.stack.is_empty() {
            let address = self.stack.pop().unwrap();
            let value = self.read_heap(address);
            self.stack.push(value);
            self.move_on()
        } else {
            Err("Can't retrieve value: not enough elements on the stack")
        }
    }

    fn run_callsubroutine(&mut self, label: &str) -> ExecutionResult {
        self.call_stack.push(self.ip);
        self.run_jump(label)
    }

    fn run_jump(&mut self, label: &str) -> ExecutionResult {
        match self.labels.get(label) {
            Some(new_ip) => {
                self.ip = *new_ip;
                Ok(())
            }
            None => Err("Can't jump: missing label !"),
        }
    }

    fn conditionnal_jump(&mut self, label: &str, predicate: impl Fn(i32) -> bool) -> ExecutionResult {
        if !self.stack.is_empty() {
            let top_stack_value = self.stack.pop().unwrap();
            if predicate(top_stack_value) {
                self.run_jump(label)
            } else {
                self.move_on()
            }
        } else {
            Err("Can't jump: stack is empty")
        }
    }    

    fn run_jzero(&mut self, label: &str) -> ExecutionResult {
        self.conditionnal_jump(label, |t| t == 0)
    }
    fn run_jneg(&mut self, label: &str) -> ExecutionResult {
        self.conditionnal_jump(label, |t| t < 0)
    }
    fn run_endofsubroutine(&mut self) -> ExecutionResult {
        if !self.call_stack.is_empty() {
            let previous_address = self.call_stack.pop().unwrap();
            self.ip = previous_address;
            self.move_on()
        } else {
            Err("Can't jump back to previous address: call stack is empty")
        }
    }

    fn run_printchar(&mut self) -> ExecutionResult {
        if !self.stack.is_empty() {
            let top_stack_value = self.stack.pop().unwrap();
            print!("{}", (top_stack_value as u8) as char);
            self.move_on()
        } else {
            Err("Can't print: stack is empty")
        }
    }
    fn run_printint(&mut self) -> ExecutionResult {
        if !self.stack.is_empty() {
            let top_stack_value = self.stack.pop().unwrap();
            print!("{}", top_stack_value);
            self.move_on()
        } else {
            Err("Can't print: stack is empty")
        }
    }
    fn run_readchar(&mut self) -> ExecutionResult {
        unimplemented!("Todo !")
    }
    fn run_readint(&mut self) -> ExecutionResult {
        if !self.stack.is_empty() {

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            println!("{:?}", input);
            let int_read: i32 = input.trim().parse().unwrap();
            println!("{:?}", int_read);

            let address = self.stack.pop().unwrap();
            self.write_heap(address, int_read);
            self.move_on()
        } else {
            Err("Can't print: stack is empty")
        }
    }

    fn write_heap(&mut self, address: i32, value: i32) {
        self.heap.resize((address+1) as usize, 0);
        self.heap[address as usize] = value;
    }

    fn read_heap(&mut self, address: i32) -> i32 {
        if (address as usize) < self.heap.len() {
            self.heap[address as usize]
        } else {
            0
        }
    }
}
