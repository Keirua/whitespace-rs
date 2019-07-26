extern crate whitespace;

use std::path::PathBuf;
use structopt::StructOpt;

use std::fs::File;
use std::io::prelude::*;
use whitespace::parser::*;

use whitespace::compiler::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "ws")]
struct Opt {
    /// Input file
    #[structopt(name = "ws_file", parse(from_os_str))]
    ws_file: PathBuf,
    #[structopt(name = "output_file", parse(from_os_str))]
    output_file: PathBuf,
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    let display = opt.ws_file.display();
    let mut file = match File::open(&opt.ws_file) {
        Err(why) => panic!("couldn't create {}: {:?}", display, why),
        Ok(file) => file,
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let instructions = parse_program(&contents);
    println!("{:?}", instructions);
    generate_source_code(&instructions.unwrap(), &opt.output_file);
    Ok(())
}
