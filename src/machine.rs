use crate::state::State;
use crate::tape::load_from_file;
use crate::tape::Tape;
use crate::tuple::Moves;
use crate::tuple::Tuple;
use itertools::Itertools;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub struct TuringMachine {
    states: Vec<State>,
    inital_state: State,
    final_states: Vec<State>,
    input_alph: Vec<char>,
    tuples: Vec<Tuple>,
    empty_space: char,
    tape: Tape,
    current_state: State,
}

impl TuringMachine {
    pub fn new(
        states: Vec<State>,
        inital_state: State,
        final_states: Vec<State>,
        input_alph: Vec<char>,
        tuples: Vec<Tuple>,
        empty_space: char,
        tape: Tape,
    ) -> TuringMachine {
        let current_state: State = State::new(inital_state.id());
        TuringMachine {
            states,
            inital_state,
            final_states,
            input_alph,
            tuples,
            empty_space,
            tape,
            current_state,
        }
    }

    pub fn run(&mut self) -> Result<&Tape, Error> {
        loop {
            let current_char: char = self.tape.get_char_at_pos();
            let tuple: Option<&Tuple> = self
                .tuples
                .iter()
                .filter(|t| (t.read_symb == current_char) && (t.state == self.current_state))
                .next();
            if tuple == None {
                break;
            }
            let tuple: Tuple = tuple.unwrap().clone();
            self.tape.set_char_at_pos(tuple.write_symb);
            self.tape.move_head(tuple._move);
            self.current_state = tuple.next_state;
        }
        if !self.final_states.contains(&self.current_state) {
            Error::new(ErrorKind::Other, "Finished in a non-final state");
        }
        Ok(&self.tape)
    }
}

/**
 * Function to create N new states
 */
fn create_states(n_states: u32) -> Vec<State> {
    (0..n_states).map(|i| State::new(i as u32)).collect()
}

fn create_final_states(ids: Vec<&str>) -> Vec<State> {
    return ids
        .into_iter()
        .map(|i| State::new(i.parse::<u32>().unwrap()))
        .collect();
}

fn create_tuple(definition: &str) -> Tuple {
    let tokens: Vec<&str> = definition.split_whitespace().collect();
    Tuple {
        state: State::new(tokens[0].parse::<u32>().unwrap()),
        read_symb: tokens[1].chars().next().expect("Read symbol is empty"),
        write_symb: tokens[2].chars().next().expect("Write symbol is empty"),
        _move: match tokens[3] {
            "S" => Moves::S,
            "R" => Moves::R,
            "L" => Moves::L,
            _ => panic!("Move not recognized!"),
        },
        next_state: State::new(tokens[4].parse::<u32>().unwrap()),
    }
}

pub fn load_from_instance(
    tm_filename: String,
    tape_filename: String,
) -> Result<TuringMachine, io::Error> {
    let file = File::open(tm_filename)?;
    let file_reader = BufReader::new(file);
    let data: Vec<String> = file_reader.lines().filter_map(io::Result::ok).collect();

    let mut states: Vec<State> = vec![];
    let mut initial_state: State = State::new(0);
    let mut final_states: Vec<State> = vec![];
    let mut tuples: Vec<Tuple> = vec![];
    let mut white_space: char = '$';
    let mut alpha: Vec<char> = vec![];

    for (index, line) in data.iter().enumerate() {
        match index {
            0 => states = create_states(line.parse::<u32>().unwrap()),
            1 => initial_state = State::new(line.parse::<u32>().unwrap()),
            2 => final_states = create_final_states(line.split(',').collect()),
            3 => white_space = line.chars().next().expect("No white space given"),
            _ => {
                let tuple: Tuple = create_tuple(line);
                alpha.push(tuple.read_symb);
                alpha.push(tuple.write_symb);
                tuples.push(tuple);
            }
        }
    }
    let alpha: Vec<char> = alpha.into_iter().unique().collect();
    let mut tape: Tape = load_from_file(tape_filename).unwrap();
    tape.set_white_char(white_space);
    let tm: TuringMachine = TuringMachine::new(
        states,
        initial_state,
        final_states,
        alpha,
        tuples,
        white_space,
        tape,
    );
    Ok(tm)
}
