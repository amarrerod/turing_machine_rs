mod state;
use state::State;

mod machine;

fn main() {
    let states: Vec<State> = (0..10).map(|i| State::new(i)).collect();
    let final_states: Vec<State> = (0..10)
        .filter(|x| x % 2 == 0)
        .map(|x| State::new(x))
        .collect();
    println!("The states are: {:?}", states);
    println!("The final states are: {:?}", final_states);
    let alph: Vec<char> = "abcdef".chars().collect();
    let tm: machine::TuringMachine =
        machine::TuringMachine::new(states, State::new(0), final_states, alph, '$');
    println!("The Turing Machine is: {:#?}", tm);
}
