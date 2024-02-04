mod sim8086;
use sim8086::{dis::Dissassembler, emu::Emulator};
use std::{
    env::args,
    fs::File,
    io::{Error, Read},
};

fn main() -> Result<(), Error> {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        panic!("You must provide a file to the path as first and only argument")
    }

    let mut in_f = File::open(&args[1])?;
    let mut program = Vec::new();
    in_f.read_to_end(&mut program)?;

    let mut sim = Dissassembler::new(&program);
    let mut emu = Emulator::new();
    while let Some(inst) = sim.get_instruction_at(emu.instruction_pointer as usize) {
        // println!("{:?}", inst);
        emu.execute_instruction(&inst);
        println!("{:?}", emu);
    }

    Ok(())
}
