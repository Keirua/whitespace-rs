extern crate whitespace;

use std::collections::HashMap;
use whitespace::instruction::*;
use whitespace::vm::*;

#[test]
fn instruction_push_nominal() {
    let mut vm = VirtualMachine::new(&vec![Instruction::Push(42)]);
    let _ = vm.run_instruction();
    assert_eq!(vec![42], vm.stack);
    assert_eq!(1, vm.ip);
}

#[test]
fn instruction_duplicate_nominal_nominal() {
    let mut vm = VirtualMachine::new(&vec![
        Instruction::Push(42),
        Instruction::Push(17),
        Instruction::Duplicate,
    ]);
    let res = vm.run_n(3);
    assert!(res.is_ok());
    assert_eq!(vec![42, 17, 17], vm.stack);
    assert_eq!(3, vm.ip);
}

#[test]
fn instruction_duplicate_error_nominal() {
    let mut vm = VirtualMachine::new(&vec![Instruction::Duplicate]);
    let res = vm.run_instruction();
    assert!(res.is_err());
}

#[test]
fn instruction_swap_nominal_nominal() {
    let mut vm = VirtualMachine::new(&vec![
        Instruction::Push(42),
        Instruction::Push(17),
        Instruction::Swap,
    ]);
    let res = vm.run_n(3);
    assert!(res.is_ok());
    assert_eq!(vec![17, 42], vm.stack);
    assert_eq!(3, vm.ip);
}

#[test]
fn instruction_swap_error_nominal() {
    let mut vm = VirtualMachine::new(&vec![Instruction::Push(42), Instruction::Swap]);
    let res = vm.run_n(2);
    assert!(res.is_err());
}

#[test]
fn instruction_discard_nominal_nominal() {
    let mut vm = VirtualMachine::new(&vec![
        Instruction::Push(42),
        Instruction::Push(17),
        Instruction::Discard,
    ]);
    let res = vm.run_n(3);
    assert!(res.is_ok());
    assert_eq!(vec![42], vm.stack);
    assert_eq!(3, vm.ip);
}

#[test]
fn instruction_discard_error_nominal() {
    let mut vm = VirtualMachine::new(&vec![Instruction::Swap]);
    let res = vm.run_instruction();
    assert!(res.is_err());
}

#[test]
fn instruction_add_nominal_nominal() {
    let mut vm = VirtualMachine::new(&vec![
        Instruction::Push(42),
        Instruction::Push(17),
        Instruction::Add,
    ]);
    let res = vm.run_n(3);
    assert!(res.is_ok());
    assert_eq!(vec![59], vm.stack);
    assert_eq!(3, vm.ip);
}

#[test]
fn instruction_add_error_nominal() {
    let mut vm = VirtualMachine::new(&vec![Instruction::Push(42), Instruction::Add]);
    let _ = vm.run_instruction();
    let res = vm.run_instruction();
    assert!(res.is_err());
}

#[test]
fn instruction_sub_nominal_nominal() {
    let mut vm = VirtualMachine::new(&vec![
        Instruction::Push(42),
        Instruction::Push(17),
        Instruction::Sub,
    ]);
    let res = vm.run_n(3);
    assert!(res.is_ok());
    assert_eq!(vec![25], vm.stack);
    assert_eq!(3, vm.ip);
}

#[test]
fn instruction_sub_error_nominal() {
    let mut vm = VirtualMachine::new(&vec![Instruction::Push(42), Instruction::Sub]);
    let res = vm.run_n(2);
    assert!(res.is_err());
}

#[test]
fn instruction_mul_nominal_nominal() {
    let mut vm = VirtualMachine::new(&vec![
        Instruction::Push(3),
        Instruction::Push(2),
        Instruction::Mul,
    ]);
    let res = vm.run_n(3);
    assert!(res.is_ok());
    assert_eq!(vec![6], vm.stack);
    assert_eq!(3, vm.ip);
}

#[test]
fn instruction_mul_error_nominal() {
    let mut vm = VirtualMachine::new(&vec![Instruction::Push(42), Instruction::Swap]);
    let res = vm.run_n(2);
    assert!(res.is_err());
}

#[test]
fn instruction_div_nominal_nominal() {
    let mut vm = VirtualMachine::new(&vec![
        Instruction::Push(6),
        Instruction::Push(3),
        Instruction::Div,
    ]);
    let res = vm.run_n(3);
    assert!(res.is_ok());
    assert_eq!(vec![2], vm.stack);
    assert_eq!(3, vm.ip);
}

#[test]
fn instruction_div_error_nominal() {
    let mut vm = VirtualMachine::new(&vec![Instruction::Push(42), Instruction::Swap]);
    let res = vm.run_n(2);
    assert!(res.is_err());
}

#[test]
fn instruction_mod_nominal_nominal() {
    let mut vm = VirtualMachine::new(&vec![
        Instruction::Push(7),
        Instruction::Push(3),
        Instruction::Mod,
    ]);
    let res = vm.run_n(3);

    assert!(res.is_ok());
    assert_eq!(vec![1], vm.stack);
    assert_eq!(3, vm.ip);
}

#[test]
fn instruction_mod_error_nominal() {
    let mut vm = VirtualMachine::new(&vec![Instruction::Push(42), Instruction::Swap]);
    let res = vm.run_n(2);
    assert!(res.is_err());
}

#[test]
fn instruction_setlabel_nominal() {
    let vm = VirtualMachine::new(&vec![
        Instruction::Push(42),
        Instruction::SetLabel("plop".to_string()),
        Instruction::Push(0),
        Instruction::Jump("plop".to_string()),
    ]);

    let mut expected_map: HashMap<String, usize> = HashMap::new();
    expected_map.insert("plop".to_string(), 1);
    assert_eq!(vm.labels, expected_map);
}

#[test]
fn instruction_jump_nominal() {
    let mut vm = VirtualMachine::new(&vec![
        Instruction::Push(42),
        Instruction::SetLabel("plop".to_string()),
        Instruction::Push(0),
        Instruction::Jump("plop".to_string()),
    ]);
    let res = vm.run_n(4);

    assert!(res.is_ok());
    assert_eq!(vm.ip, 1);
    assert_eq!(vm.stack, vec![42, 0]);
}

#[test]
fn instruction_jump_no_zero_test() {
    let mut vm = VirtualMachine::new(&vec![
        Instruction::Push(0),
        Instruction::SetLabel("plop".to_string()),
        Instruction::Push(42),
        Instruction::Jump("plop".to_string()),
    ]);
    let res = vm.run_n(4);

    assert!(res.is_ok());
    assert_eq!(vm.ip, 1);
    assert_eq!(vm.stack, vec![0, 42]);
}

#[test]
fn instruction_jump_missing_label_test() {
    let mut vm = VirtualMachine::new(&vec![
        Instruction::Push(42),
        Instruction::Push(0),
        Instruction::Jump("plop".to_string()),
    ]);
    let res = vm.run_n(3);

    assert!(res.is_err());
    assert_eq!(vm.stack, vec![42, 0]);
}

#[test]
fn instruction_jump_empty_stack_test() {
    let mut vm = VirtualMachine::new(&vec![
        Instruction::SetLabel("plop".to_string()),
        Instruction::Jump("plop".to_string()),
    ]);
    let res = vm.run_n(2);

    assert!(res.is_ok());
    assert_eq!(vm.stack, vec![]);
    assert_eq!(vm.ip, 0);
}

#[test]
fn instruction_jzero_nominal() {
    let mut vm = VirtualMachine::new(&vec![
        Instruction::Push(42),
        Instruction::SetLabel("plop".to_string()),
        Instruction::Push(0),
        Instruction::JZero("plop".to_string()),
    ]);
    let res = vm.run_n(4);

    assert!(res.is_ok());
    assert_eq!(vm.ip, 1);
    assert_eq!(vm.stack, vec![42]);
}

#[test]
fn instruction_jzero_no_jump_test() {
    let mut vm = VirtualMachine::new(&vec![
        Instruction::Push(0),
        Instruction::SetLabel("plop".to_string()),
        Instruction::Push(42),
        Instruction::JZero("plop".to_string()),
    ]);
    let res = vm.run_n(4);

    assert!(res.is_ok());
    assert_eq!(vm.ip, 4);
    assert_eq!(vm.stack, vec![0]);
}

#[test]
fn instruction_jzero_missing_label_test() {
    let mut vm = VirtualMachine::new(&vec![
        Instruction::Push(42),
        Instruction::Push(0),
        Instruction::JZero("plop".to_string()),
    ]);
    let res = vm.run_n(3);

    assert!(res.is_err());
    assert_eq!(vm.stack, vec![42]);
}

#[test]
fn instruction_jzero_empty_stack_test() {
    let mut vm = VirtualMachine::new(&vec![
        Instruction::SetLabel("plop".to_string()),
        Instruction::JZero("plop".to_string()),
    ]);
    let res = vm.run_n(3);

    assert!(res.is_err());
    assert_eq!(vm.stack, vec![]);
}

#[test]
fn instruction_jneg_nominal() {
    let mut vm = VirtualMachine::new(&vec![
        Instruction::Push(42),
        Instruction::SetLabel("plop".to_string()),
        Instruction::Push(-2),
        Instruction::JNeg("plop".to_string()),
    ]);
    let res = vm.run_n(4);

    assert!(res.is_ok());
    assert_eq!(vm.ip, 1);
    assert_eq!(vm.stack, vec![42]);
}

#[test]
fn instruction_jneg_no_jump_test() {
    let mut vm = VirtualMachine::new(&vec![
        Instruction::Push(-2),
        Instruction::SetLabel("plop".to_string()),
        Instruction::Push(42),
        Instruction::JNeg("plop".to_string()),
    ]);
    let res = vm.run_n(4);

    assert!(res.is_ok());
    assert_eq!(vm.ip, 4);
    assert_eq!(vm.stack, vec![-2]);
}

#[test]
fn instruction_jneg_missing_label_test() {
    let mut vm = VirtualMachine::new(&vec![
        Instruction::Push(42),
        Instruction::Push(-2),
        Instruction::JNeg("plop".to_string()),
    ]);
    let res = vm.run_n(3);

    assert!(res.is_err());
    assert_eq!(vm.stack, vec![42]);
}

#[test]
fn instruction_jneg_empty_stack_test() {
    let mut vm = VirtualMachine::new(&vec![
        Instruction::SetLabel("plop".to_string()),
        Instruction::JNeg("plop".to_string()),
    ]);
    let res = vm.run_n(3);

    assert!(res.is_err());
    assert_eq!(vm.stack, vec![]);
}

#[test]
fn instruction_call_subroutine_nominal() {
    let mut vm = VirtualMachine::new(&vec![
        Instruction::Push(42),
        Instruction::SetLabel("plop".to_string()),
        Instruction::Push(-2),
        Instruction::CallSubroutine("plop".to_string()),
    ]);
    let res = vm.run_n(4);

    assert!(res.is_ok());
    assert_eq!(vm.ip, 1);
    assert_eq!(vm.stack, vec![42, -2]);
    assert_eq!(vm.call_stack, vec![3]);
}

#[test]
fn instruction_call_subroutine_no_jump_test() {
    let mut vm = VirtualMachine::new(&vec![
        Instruction::Push(-2),
        Instruction::SetLabel("plop".to_string()),
        Instruction::Push(42),
        Instruction::CallSubroutine("plop".to_string()),
    ]);
    let res = vm.run_n(4);

    assert!(res.is_ok());
    assert_eq!(vm.ip, 1);
    assert_eq!(vm.stack, vec![-2, 42]);
    assert_eq!(vm.call_stack, vec![3]);
}

#[test]
fn instruction_call_subroutine_missing_label_test() {
    let mut vm = VirtualMachine::new(&vec![
        Instruction::Push(42),
        Instruction::Push(-2),
        Instruction::CallSubroutine("plop".to_string()),
    ]);
    let res = vm.run_n(3);

    assert!(res.is_err());
    assert_eq!(vm.stack, vec![42, -2]);
    assert_eq!(vm.call_stack, vec![2]);
}

#[test]
fn instruction_call_subroutine_empty_stack_test() {
    let mut vm = VirtualMachine::new(&vec![
        Instruction::SetLabel("plop".to_string()),
        Instruction::CallSubroutine("plop".to_string()),
    ]);
    let res = vm.run_n(2);

    assert!(res.is_ok());
    assert_eq!(vm.stack, vec![]);
    assert_eq!(vm.call_stack, vec![1]);
}

#[test]
fn instruction_store_nominal_test() {
    let mut vm = VirtualMachine::new(&vec![
        Instruction::Push(42),
        Instruction::Push(-2),
        Instruction::Store,
    ]);
    let res = vm.run_n(3);

    assert!(res.is_ok());
    assert_eq!(vm.stack, vec![]);
    assert_eq!(vm.heap[42], -2);
}

#[test]
fn instruction_store_can_resize_test() {
    let mut vm = VirtualMachine::new(&vec![
        Instruction::Push(42),
        Instruction::Push(-2),
        Instruction::Store,
        Instruction::Push(128),
        Instruction::Push(17),
        Instruction::Store,
    ]);
    let res = vm.run_n(6);

    assert!(res.is_ok());
    assert_eq!(vm.stack, vec![]);
    assert_eq!(vm.heap[42], -2);
    assert_eq!(vm.heap[128], 17);
}

#[test]
fn instruction_store_not_enough_args_test() {
    let mut vm = VirtualMachine::new(&vec![Instruction::Push(42), Instruction::Store]);
    let res = vm.run_n(3);

    assert!(res.is_err());
}

#[test]
fn instruction_retrieve_nominal_test() {
    let mut vm = VirtualMachine::new(&vec![
        Instruction::Push(42),
        Instruction::Push(-2),
        Instruction::Store,
        Instruction::Push(42),
        Instruction::Retrieve,
    ]);
    let res = vm.run_n(5);

    assert!(res.is_ok());
    assert_eq!(vm.heap[42], -2);
    assert_eq!(vm.stack, vec![-2]);
    assert_eq!(43, vm.heap.len());
}

#[test]
fn instruction_retrieve_missing_address_test() {
    let mut vm = VirtualMachine::new(&vec![
        Instruction::Push(128),
        Instruction::Push(-2),
        Instruction::Store,
        Instruction::Push(42),
        Instruction::Retrieve,
    ]);
    let res = vm.run_n(5);

    assert!(res.is_ok());
    assert_eq!(vm.stack, vec![0]); // default value
    assert_eq!(vm.heap[42], 0);
    assert_eq!(vm.heap[128], -2);
    assert_eq!(128 + 1, vm.heap.len());
}

#[test]
fn instruction_retrieve_address_outside_bounds_test() {
    let mut vm = VirtualMachine::new(&vec![
        Instruction::Push(42),
        Instruction::Push(-2),
        Instruction::Store,
        Instruction::Push(128),
        Instruction::Retrieve,
    ]);
    let res = vm.run_n(5);

    assert!(res.is_ok());
    assert_eq!(vm.stack, vec![0]); // default value
    assert_eq!(vm.heap[42], -2);
    assert_eq!(43, vm.heap.len());
}

#[test]
fn instruction_retrieve_not_enough_args_test() {
    let mut vm = VirtualMachine::new(&vec![
        Instruction::Push(42),
        Instruction::Push(17),
        Instruction::Store,
        Instruction::Retrieve,
    ]);
    let res = vm.run_n(4);

    assert!(res.is_err());
    assert_eq!(43, vm.heap.len());
}


#[test]
fn running_an_empty_program_does_not_crash_test() {
    let mut vm = VirtualMachine::new(&vec![]);
    vm.run_program();
}
