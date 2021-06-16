mod state;
use state::State;

fn main() {
    let states : Vec<State> = (0..10).map(|i| State::new(i)).collect();
    println!("The states are: {:?}", states);

}
