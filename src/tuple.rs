use crate::state::State;

#[derive(Debug)]
pub enum Moves {
    L,
    R,
    S,
}

#[derive(Debug)]
pub struct Tuple {
    pub state: State,
    pub read_symb: char,
    pub write_symb: char,
    pub _move: Moves,
    pub next_state: State,
}

impl Tuple {
    pub fn new(
        state: State,
        read_symb: char,
        write_symb: char,
        _move: Moves,
        next_state: State,
    ) -> Tuple {
        Tuple {
            state,
            read_symb,
            write_symb,
            _move,
            next_state,
        }
    }
}
