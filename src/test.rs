use crate::state::State;
use crate::tuple::Moves;
use crate::tuple::Tuple;
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
