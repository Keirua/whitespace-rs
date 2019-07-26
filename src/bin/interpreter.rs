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
    #[structopt(name = "debug", short, long)]
    debug: bool,
    /// Display the parsed instructions
    #[structopt(name = "instructions", short, long)]
    instructions: bool,
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
    match instructions {
        Ok(instructions) => {
            if opt.instructions {
                println!("{:#?}", instructions);
            }

            let mut vm = VirtualMachine::new(&instructions);
            vm.debug = opt.debug;

            //let _ = vm.run_n(30);
            vm.run_program();
        },
        Err(e) => println!("{:?}", e)
    }
    Ok(())
}
