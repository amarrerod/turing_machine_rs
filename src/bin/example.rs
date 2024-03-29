use std::env;
use turing_machine::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.iter().count() != 3 {
        panic!("No Turing Machine file (.tm) or Tape file (.tape) was given");
    }
    let mut tm: TuringMachine =
        utils::load_from_instance(args[1].to_string(), args[2].to_string())
            .expect("Error loading TM");
    let tape: &tape::Tape = tm.run().unwrap();
    println!("The result is: {:?}", tape);
}
