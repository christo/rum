use std::env;
use std::fs::File;
use std::io::{self, Read};

#[repr(u8)]
enum Instructions {
    ConditionalMove = 0,
    ArrayIndex,
    ArrayAmendment,
    Addition,
    Multiplication,
    Division,
    NotAnd,
    Halt,
    Allocation,
    Abandonment,
    Output,
    Input,
    LoadProgram,
    Orthography,
}


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage {} <file_path>", args[0]);
        std::process::exit(1);
    }
    let mut file = File::open(&args[1])?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    print!("{} bytes read\n", buffer.len());
    Ok(())
}
