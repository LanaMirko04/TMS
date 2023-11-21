pub mod turing_machine;
pub mod tui;
pub mod app;

use std::io;
use clap::Parser;
use crate::app::App;

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

fn main() -> std::result::Result<(), io::Error> {
    let args = Args::parse();
    let conf = args.conf;
    let mut app = App::new(conf)?;
    let result = app.run();

    if let Err(err) = result {
        eprintln!("{err:?}");
    }
    Ok(())
}
