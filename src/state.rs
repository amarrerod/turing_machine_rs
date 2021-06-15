
struct State {
    id: u32
}

impl State {
    pub fn new(id : i32) -> State {
        State {
            id
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }
}