
use crate::tuple::Tuple as Tuple;
use crate::state::State as State;
use crate::tuple::Moves;
#[cfg(test)]

#[test]
fn tuple_creating() {
    let tuple: Tuple = Tuple::new(State::new(0), 'A', 'B', Moves::S, State::new(1));
    assert_eq!(tuple.read_symb, 'A');
}
