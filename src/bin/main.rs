extern crate whitespace;

use std::path::PathBuf;
use structopt::StructOpt;

use std::fs::File;
use std::io::prelude::*;
use whitespace::parser::*;
use whitespace::vm::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "ws")]
struct Opt {
    #[structopt(short, long)]
    debug: bool,
    /// Input file
    #[structopt(name = "FILE", parse(from_os_str))]
    file: PathBuf,
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();

    let mut file = File::open(opt.file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let instructions = parse_program(&contents);
    if opt.debug {
        println!("{:#?}", instructions);
    }

    let mut vm = VirtualMachine::new(&instructions.unwrap());
    vm.debug = opt.debug;

    //let _ = vm.run_n(30);
    vm.run_program();
    Ok(())
}
