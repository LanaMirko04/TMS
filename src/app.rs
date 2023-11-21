use std::{error::Error, io};
use crate::turing_machine::TuringMachine;
use crate::tui;

pub struct App {
    running: bool,
    conf: String,
    tm: TuringMachine,
}

impl App {
    pub fn new(conf_path: String) -> Result<Self, io::Error> {
        let mut tm = TuringMachine::new();
        tm.load_cfg(&conf_path)?;

        Ok(Self {
           running: false,
           conf: conf_path,
           tm,
        })
    }

    pub fn run(&mut self) -> Result<(), io::Error> {
        let mut terminal = tui::setup_terminal()?;

        'outer: loop {
            terminal.draw(|f| tui::ui(f, &&self.tm))?;

            match tui::handle_events()? {
                tui::TuiEvent::Run => self.running = true,
                tui::TuiEvent::Pause => self.running = false,
                tui::TuiEvent::Step => self.tm.step(),
                tui::TuiEvent::Reset => self.tm.load_cfg(&self.conf)?,
                tui::TuiEvent::Quit => break 'outer,
                tui::TuiEvent::NoEvent => {}
            }

            if self.running {
                self.tm.step();
            }
        }

        tui::restore_terminal(terminal)?;

        Ok(())
    }
}
