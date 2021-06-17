use std::env;
mod machine;
mod state;
mod tape;
mod tuple;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.iter().count() != 3 {
        panic!("No Turing Machine file (.tm) or Tape file (.tape) was given");
    }
    let tm: machine::TuringMachine =
        machine::load_from_instance(args[1].to_string()).expect("Error loading TM");
    tape::load_from_file(args[2].to_string());

    println!("The Turing Machine is: {:#?}", tm);
}
