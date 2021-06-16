

use crate::state::State;
use crate::tuple::Tuple;
use crate::tuple::Moves;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
pub struct TuringMachine {
    states: Vec<State>,
    inital_state: State,
    final_states: Vec<State>,
    input_alph: Vec<char>,
    empty_space: char,
}

impl TuringMachine {
    pub fn new(
        states: Vec<State>,
        inital_state: State,
        final_states: Vec<State>,
        input_alph: Vec<char>,
        empty_space: char,
    ) -> TuringMachine {
        TuringMachine {
            states,
            inital_state,
            final_states,
            input_alph,
            empty_space,
        }
    }
}

/**
 * Function to create N new states
 */
fn create_states(n_states: u32) -> Vec<State> {
    (0..n_states).map(|i| State::new(i as u32)).collect()
}

fn create_tuple(definition: &str) -> Tuple{
    let tokens : Vec<&str>= definition.split_whitespace().collect();
    println!("The tokens are: {:?}", tokens);
    Tuple {
        state: State::new(tokens[0].parse::<u32>().unwrap()),
        read_symb: tokens[1].chars().next().expect("Read symbol is empty"),
        write_symb: tokens[2].chars().next().expect("Write symbol is empty"),
        _move: match tokens[3] {
            "S" => Moves::S,
            "R" => Moves::R,
            "L" => Moves::L,
            _ => panic!("Move not recognized!")
        },
        next_state: State::new(tokens[4].parse::<u32>().unwrap())
    }
}

pub fn load_from_instance(filename: &str) -> Result<u32, io::Error> {
    let tm: TuringMachine;
    let file = File::open(filename)?;
    let file_reader = BufReader::new(file);
    let data: Vec<String> = file_reader.lines().filter_map(io::Result::ok).collect();

    let mut states: Vec<State> = vec![];
    let mut initial_state: State;
    let mut final_states: Vec<State> = vec![];
    let mut tuples: Vec<Tuple> = vec![];
    for (index, line) in data.iter().enumerate() {
        match index {
            0 => states = create_states(line.parse::<u32>().unwrap()),
            1 => initial_state = State::new(line.parse::<u32>().unwrap()),
            2 => final_states = create_states(line.parse::<u32>().unwrap()),
            3 => continue,
            _ => tuples.push(create_tuple(line)) 
        }
    }
    println!("The tuples are: {:#?}", tuples);
    Ok(0)
}
