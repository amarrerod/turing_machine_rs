use crate::state::State;
use crate::tape::Tape;
use crate::tuple::Tuple;
use std::io::{Error, ErrorKind};

#[allow(dead_code)]
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


