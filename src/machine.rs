
    
    pub struct TuringMachine {
        states: Vec<State>,
        inital_state: State,
        final_states: Vec<State>,
        input_alph: Vec<char>,
        empty_space: char,
    }

    impl TuringMachine {
        pub fn new(
            states: Vec<State>,
            inital_state: State,
            final_states: Vec<State>,
            input_alph: Vec<char>,
            empty_space: char,
        ) -> TuringMachine {
            TuringMachine {
                states,
                inital_state,
                final_states,
                input_alph,
                empty_space,
            }
        }
}