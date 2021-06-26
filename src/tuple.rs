use crate::state::State;

#[derive(Debug, PartialEq, Clone)]
pub enum Moves {
    L,
    R,
    S,
}

#[derive(Debug, PartialEq, Clone)]
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tuple_creation() {
        let tuple: Tuple = Tuple::new(State::new(0), 'A', 'B', Moves::S, State::new(1));
        assert_eq!(tuple.read_symb, 'A');
        assert_eq!(tuple.write_symb, 'B');
        assert_eq!(tuple.state, State::new(0));
        assert_eq!(tuple._move, Moves::S);
        assert_eq!(tuple.next_state, State::new(1));
    }
}

