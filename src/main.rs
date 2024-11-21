use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

type Platter = u32;
type ArrayID = u32;

#[derive(Default)]
struct UniversalMachine {
    regs: [Platter; 8],
    arrays: HashMap<ArrayID, Vec<Platter>>,
    free_ids: Vec<ArrayID>,
    execution_finger: usize,
    program: Vec<Platter>,
}

const CONDITIONAL_MOVE: u32 = 0;
const ARRAY_INDEX: u32 = 1;
const ARRAY_AMENDMENT: u32 = 2;
const ADDITION: u32 = 3;
const MULTIPLICATION: u32 = 4;
const DIVISION: u32 = 5;
const NOT_AND: u32 = 6;
const HALT: u32 = 7;
const ALLOCATION: u32 = 8;
const ABANDONMENT: u32 = 9;
const OUTPUT: u32 = 10;
const INPUT: u32 = 11;
const LOAD_PROGRAM: u32 = 12;
const ORTHOGRAPHY: u32 = 13;

impl UniversalMachine {
    pub fn new(program: Vec<Platter>) -> Self {
        let mut um = UniversalMachine::default();
        um.arrays.insert(0, program.clone());
        um.program = program;
        um
    }

    pub fn run(&mut self) {
        loop {
            if self.execution_finger >= self.program.len() {
                panic!("Execution finger out of bounds");
            }

            let instruction = self.program[self.execution_finger];
            self.execution_finger += 1;
            let a = ((instruction >> 6) & 0x7) as usize;
            let b = ((instruction >> 3) & 0x7) as usize;
            let c = (instruction & 0x7) as usize;
            let operator = instruction >> 28;
            match operator {
                CONDITIONAL_MOVE => {
                    if self.regs[c] != 0 {
                        self.regs[a] = self.regs[b];
                    }
                }
                ARRAY_INDEX => {
                    let array = self
                        .arrays
                        .get(&self.regs[b])
                        .expect("Array not found");
                    self.regs[a] = array[self.regs[c] as usize];
                }
                ARRAY_AMENDMENT => {
                    let array = self
                        .arrays
                        .get_mut(&self.regs[a])
                        .expect("Array not found");
                    array[self.regs[b] as usize] = self.regs[c];
                }
                ADDITION => {
                    self.regs[a] = self.regs[b].wrapping_add(self.regs[c]);
                }
                MULTIPLICATION => {
                    self.regs[a] = self.regs[b].wrapping_mul(self.regs[c]);
                }
                DIVISION => {
                    if self.regs[c] == 0 {
                        panic!("Division by zero");
                    }
                    self.regs[a] = self.regs[b] / self.regs[c];
                }
                NOT_AND => {
                    self.regs[a] = !(self.regs[b] & self.regs[c]);
                }
                HALT => break, // Halt
                ALLOCATION => {
                    let size = self.regs[c] as usize;
                    let id = if let Some(id) = self.free_ids.pop() {
                        id
                    } else {
                        self.arrays.len() as ArrayID
                    };
                    self.arrays.insert(id, vec![0; size]);
                    self.regs[b] = id;
                }
                ABANDONMENT => {
                    let id = self.regs[c];
                    if id == 0 {
                        panic!("Cannot abandon array 0");
                    } else if !self.arrays.contains_key(&id) {
                        panic!("Invalid array abandonment, no array with ID {} found", id);
                    }
                    self.arrays.remove(&id);
                    self.free_ids.push(id);
                }
                OUTPUT => {
                    let value = self.regs[c];
                    if value > 255 {
                        panic!("Output value out of range");
                    }
                    let value = self.regs[c];
                    if value > 255 {
                        panic!("Output value out of range");
                    }
                    // Write the raw byte directly to stdout
                    io::stdout()
                        .write_all(&[value as u8])
                        .expect("Failed to write output");
                    io::stdout().flush().expect("Failed to flush output");
                }
                INPUT => {
                    let mut buffer = [0; 1];
                    if let Ok(_) = io::stdin().read_exact(&mut buffer) {
                        self.regs[c] = buffer[0] as Platter;
                    } else {
                        self.regs[c] = u32::MAX;
                    }
                }
                LOAD_PROGRAM => {
                    if self.regs[b] != 0 {
                        let program = self
                            .arrays
                            .get(&self.regs[b])
                            .expect("Array not found")
                            .clone();
                        self.arrays.insert(0, program.clone());
                        self.program = program;
                    }
                    self.execution_finger = self.regs[c] as usize;
                }
                ORTHOGRAPHY => {
                    let a = ((instruction >> 25) & 0x7) as usize;
                    let value = instruction & 0x1FFFFFF;
                    self.regs[a] = value;
                }
                _ => panic!("Invalid operator: {}", operator),
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <program-file>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];

    let program = match read_program_file(filename) {
        Ok(program) => program,
        Err(err) => {
            eprintln!("Error reading program file '{}': {}", filename, err);
            std::process::exit(1);
        }
    };

    let mut um = UniversalMachine::new(program);
    um.run();
}

/// Reads a program binary file and returns a vector of platters
fn read_program_file(filename: &str) -> io::Result<Vec<Platter>> {
    let mut file = File::open(filename)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    if buffer.len() % 4 != 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "File size is not a multiple of 4 bytes",
        ));
    }

    let program = buffer
        .chunks(4)
        .map(|chunk| {
            let mut platter = [0; 4];
            platter.copy_from_slice(chunk);
            Platter::from_be_bytes(platter)
        })
        .collect();

    Ok(program)
}
