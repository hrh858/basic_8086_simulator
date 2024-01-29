mod v2_sim8086;
use std::{
    env::args,
    fs::File,
    io::{Error, Read},
};
use v2_sim8086::dis::Dissassembler;

// Hello rsync!

fn main() -> Result<(), Error> {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        panic!("You must provide a file to the path as first and only argument")
    }

    let mut in_f = File::open(&args[1])?;
    let mut program = Vec::new();
    in_f.read_to_end(&mut program)?;

    // let filename = &args[1].split('/').last().unwrap();
    // let mut out_f = File::create(format!("{}.asm", &filename.split(".").nth(0).unwrap()))?;

    // out_f.write("bits 16\n\n".as_bytes())?;

    let mut sim = Dissassembler::new(&program);
    while let Some(inst) = sim.next() {
        println!("{:?}", inst)
    }

    // out_f.write(&format!("{}", instruction).into_bytes())?;

    Ok(())
}
