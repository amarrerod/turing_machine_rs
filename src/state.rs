/**
 * Struct which represents a states in the TM
 */
#[derive(Debug, PartialEq, Clone)]
pub struct State {
    id: u32,
}

impl State {
    pub fn new(id: u32) -> State {
        State { id }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn state_creation() {
        for i in 0..100 {
            let state: State = State::new(i);
            assert_eq!(state.id(), i);
            assert_ne!(state, State::new(i + 1));
        }
    }
}
