mod turing_machine;

use std::io;
use clap::Parser;
use crate::turing_machine::TuringMachine;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(short,
          long,
          value_name="CONF_PATH",
          help="TMS configuration file path"
    )]
    conf: String,
}

fn main() -> Result<(), io::Error> {
    let args = Args::parse();
    let conf = args.conf;
    let mut tm = TuringMachine::new();
    
    if let Err(error) = tm.load_cfg(&conf) {
        panic!("Error: {}", error);
    }

    tm.print_tape();
    while !tm.is_halt() {
        tm.step();
        tm.print_tape()
    }

    Ok(())
}
