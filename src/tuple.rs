
use state::State;
use moves::Moves;

pub struct Tuple {
    state: State,
    read_symb: char,
    write_symb: char,
    _move: Moves,
    next_state: State
}

impl Tuple {
    pub fn new(state: State, read_symb: char, write_symb: char, _move: Move, next_state: State) {
        Tuple {
            state,
            read_symb,
            write_symb,
            _move,
            next_state
        }
    }
}