use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

type Platter = u32;
type ArrayID = u32;

#[derive(Default)]
struct UniversalMachine {
    registers: [Platter; 8],
    arrays: HashMap<ArrayID, Vec<Platter>>,
    free_ids: Vec<ArrayID>,
    execution_finger: usize,
    program: Vec<Platter>,
}

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
                0 => {
                    if self.registers[c] != 0 {
                        self.registers[a] = self.registers[b];
                    }
                }
                1 => {
                    let array = self
                        .arrays
                        .get(&self.registers[b])
                        .expect("Array not found");
                    self.registers[a] = array[self.registers[c] as usize];
                }
                2 => {
                    let array = self
                        .arrays
                        .get_mut(&self.registers[a])
                        .expect("Array not found");
                    array[self.registers[b] as usize] = self.registers[c];
                }
                3 => {
                    self.registers[a] = self.registers[b].wrapping_add(self.registers[c]);
                }
                4 => {
                    self.registers[a] = self.registers[b].wrapping_mul(self.registers[c]);
                }
                5 => {
                    if self.registers[c] == 0 {
                        panic!("Division by zero");
                    }
                    self.registers[a] = self.registers[b] / self.registers[c];
                }
                6 => {
                    self.registers[a] = !(self.registers[b] & self.registers[c]);
                }
                7 => break, // Halt
                8 => {
                    let size = self.registers[c] as usize;
                    let id = if let Some(id) = self.free_ids.pop() {
                        id
                    } else {
                        self.arrays.len() as ArrayID
                    };
                    self.arrays.insert(id, vec![0; size]);
                    self.registers[b] = id;
                }
                9 => {
                    let id = self.registers[c];
                    if id == 0 {
                        panic!("Cannot abandon array 0");
                    } else if !self.arrays.contains_key(&id) {
                        panic!("Invalid array abandonment, no array with ID {} found", id);
                    }
                    self.arrays.remove(&id);
                    self.free_ids.push(id);
                }
                10 => {
                    let value = self.registers[c];
                    if value > 255 {
                        panic!("Output value out of range");
                    }
                    let value = self.registers[c];
                    if value > 255 {
                        panic!("Output value out of range");
                    }
                    // Write the raw byte directly to stdout
                    std::io::stdout()
                        .write_all(&[value as u8])
                        .expect("Failed to write output");
                    std::io::stdout().flush().expect("Failed to flush output");
                }
                11 => {
                    let mut buffer = [0; 1];
                    if let Ok(_) = std::io::stdin().read_exact(&mut buffer) {
                        self.registers[c] = buffer[0] as Platter;
                    } else {
                        self.registers[c] = u32::MAX;
                    }
                }
                12 => {
                    if self.registers[b] != 0 {
                        let program = self
                            .arrays
                            .get(&self.registers[b])
                            .expect("Array not found")
                            .clone();
                        self.arrays.insert(0, program.clone());
                        self.program = program;
                    }
                    self.execution_finger = self.registers[c] as usize;
                }
                13 => {
                    let a = ((instruction >> 25) & 0x7) as usize;
                    let value = instruction & 0x1FFFFFF;
                    self.registers[a] = value;
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
