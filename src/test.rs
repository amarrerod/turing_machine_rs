
use crate::tuple::Tuple as Tuple;
use crate::state::State as State;
use crate::tuple::Moves;
#[cfg(test)]

#[test]
fn tuple_creating() {
    let tuple: Tuple = Tuple::new(State::new(0), 'A', 'B', Moves::S, State::new(1));
    assert_eq!(tuple.read_symb, 'A');
    assert_eq!(tuple.write_symb, 'B');
    assert_eq!(tuple.state, State::new(0));
    assert_eq!(tuple._move, Moves::S);
    assert_eq!(tuple.next_state, State::new(1));
}
