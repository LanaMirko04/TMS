mod turing_machine;

use std::io;
use crate::turing_machine::TuringMachine;

fn main() -> Result<(), io::Error> {
    let cfg_path = "/Users/mirko/Projects/tms/examples/example.cfg";
    let mut tm = TuringMachine::new();
    
    if let Err(error) = tm.load_cfg(cfg_path) {
        panic!("Error: {}", error);
    }

    tm.print_tape();
    while !tm.is_halt() {
        tm.step();
        tm.print_tape()
    }

    Ok(())
}
