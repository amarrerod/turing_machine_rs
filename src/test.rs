
use crate::tuple::Tuple as Tuple;
use crate::state::State as State;
use crate::tuple::Moves;
use crate::tape::Tape as Tape;
use crate::tape::load_from_file;
use std::io;

#[cfg(test)]

#[test]
fn tuple_creation() {
    let tuple: Tuple = Tuple::new(State::new(0), 'A', 'B', Moves::S, State::new(1));
    assert_eq!(tuple.read_symb, 'A');
    assert_eq!(tuple.write_symb, 'B');
    assert_eq!(tuple.state, State::new(0));
    assert_eq!(tuple._move, Moves::S);
    assert_eq!(tuple.next_state, State::new(1));
}

#[test]
fn state_creation() {
    for i in 0..100 {
        let state: State = State::new(i);
            assert_eq!(state.id(), i);
            assert_ne!(state, State::new(i + 1));
    }
}


#[test]
fn tape_load_from_file() {
    let filename : String = String::from("notfound.tape");
    assert_eq!(load_from_file(filename).is_err(), true);
    let filename : String = String::from("tapes/example1.tape");
    assert_eq!(load_from_file(filename).is_err(), false);
    let sample_tape: Tape = Tape::new(
        "aaaabbbb".chars().collect(),
        '$'
    );
    let tape: Tape = load_from_file(String::from("tapes/example1.tape")).expect("Error creating Tape");
    assert_eq!(sample_tape, tape);
}