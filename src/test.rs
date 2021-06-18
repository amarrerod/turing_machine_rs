
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

#[test]
fn tape_methods() {
    let mut tape: Tape = load_from_file(String::from("tapes/example1.tape")).expect("Error creating Tape");
    assert_eq!(0, tape.get_pos());
    let new_char: char = '%';
    tape.set_white_char(new_char);
    assert_eq!(new_char, tape.get_white_char());
    assert_eq!('a', tape.get_char_at_pos());
    tape.set_char_at_pos(new_char);
    assert_eq!(new_char, tape.get_char_at_pos());
    let mut new_content = tape.get_content();
    let new_tape_length : usize = new_content.len() + 1;
    new_content.push(tape.get_white_char());
    tape.update_pos(new_tape_length as i32);
    assert_eq!(new_tape_length, tape.get_content().len());
    assert_eq!(new_content, tape.get_content());
}