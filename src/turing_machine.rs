use std::fs::File;
use std::io::{self, BufRead};

#[derive(Copy, Clone, Debug)]
enum Direction {
    LHS,
    RHS,
    STAY,
}

impl Direction {
    fn str2dir(strdir: &str) -> Result<Direction, io::Error> {
        match strdir {
            "left" => Ok(Direction::LHS),
            "right" => Ok(Direction::RHS),
            "stay" => Ok(Direction::STAY),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid direction")),
        }
    }

    fn dir2str(dir: &Direction) -> String {
        match dir {
            Direction::LHS => "left".to_string(),
            Direction::RHS => "right".to_string(),
            Direction::STAY => "stay".to_string(),
        }
    }
}

struct Instruction {
    current_state: String,
    current_symbol: char,
    new_state: String,
    new_symbol: char,
    direction: Direction,
}

impl Instruction {
    fn is_matching(&self, state: &str, symbol: char) -> bool {
        return state == self.current_state && symbol == self.current_symbol
    }

    fn to_string(&self) -> String {
        format!("instruction {{current_state: {}, current_symbol: {}, new_state: {}, new_symbol: {}, direction: {}}}",
                self.current_state,
                self.current_symbol,
                self.new_state,
                self.new_symbol,
                Direction::dir2str(&self.direction))
    }
}

pub struct TuringMachine {
    state: String,
    halt_state: String,
    tape: Vec<char>,
    tape_cell: usize,
    instructions: Vec<Instruction>,
}

impl TuringMachine {
    pub fn new() -> Self {
        Self {
            state: String::from(""),
            halt_state: String::from(""),
            tape: Vec::new(),
            tape_cell: 0,
            instructions: Vec::new(),
        }
    }

    pub fn load_cfg(&mut self, path: &str) -> Result<(), io::Error> {
        let file = File::open(path).unwrap(); 
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            if line.starts_with('#') || line.trim().is_empty() {
                continue;
            }

            let substrings: Vec<&str> = line.split_whitespace().collect();
            let count = substrings.len();

            match count {
                2 => {
                    self.state = substrings[0].to_string();
                    self.tape = substrings[1].chars().collect();
                }
                1 => {
                    self.halt_state = substrings[0].to_string();
                }
                5 => {
                    let current_state = substrings[0].to_string();
                    let current_symbol = substrings[1].chars().next().unwrap();
                    let new_state = substrings[2].to_string();
                    let new_symbol = substrings[3].chars().next().unwrap();
                    let direction = Direction::str2dir(substrings[4])
                        .unwrap();

                    self.instructions.push(Instruction {
                        current_state,
                        current_symbol,
                        new_state,
                        new_symbol,
                        direction,
                    });
                }
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Invalid line in CFG file.",
                    ));
                }
            }
        }

        Ok(())
    }

    pub fn step(&mut self) {
        for i in 0..self.instructions.len() {
            if self.instructions[i]
                .is_matching(&self.state, self.tape[self.tape_cell]) {
                let state: String = self.instructions[i].new_state.clone();
                let symbol: char = self.instructions[i].new_symbol;
                let direction: Direction = self.instructions[i].direction;

                self.update(&state, symbol, direction); 
                break;
            }
        }
    }

    pub fn is_halt(&self) -> bool {
        return self.state == self.halt_state
    }

    /* TEMPORANY METHOD (Test purpose only) */
    pub fn print_tape(&self) {
        println!("{:?}", self.tape);
    }

    pub fn print_info(&self) {
        println!("State: {}", self.state);
        println!("Halt state: {}", self.halt_state);
        println!("Tape: {:?}", self.tape);
        println!("Tape cell: {}", self.tape_cell);
        println!("Instructions:");
        for (i, instruction) in self.instructions.iter().enumerate() {
            println!("#{}: {}", i + 1, instruction.to_string());
        }
    }

    fn update(&mut self, new_state: &str, new_symbol: char, dir: Direction) {
        self.state = new_state.to_string();
        self.tape[self.tape_cell] = new_symbol;

        match dir {
            Direction::LHS => self.tape_cell -= 1,
            Direction::RHS => self.tape_cell += 1,
            Direction::STAY => {},
        }
    }
}
