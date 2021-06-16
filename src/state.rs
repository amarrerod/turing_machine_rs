
/**
 * Struct which represents a states in the TM
 */
#[derive(Debug)]
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
