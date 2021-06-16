use std::env;
mod machine;
mod tuple;
mod state;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.iter().count() != 2{
        panic!("No Turing Machine file (.tm) was given");
    }
    let tm: machine::TuringMachine = machine::load_from_instance(args[1].to_string())
    .expect("Error loading TM");
    println!("The Turing Machine is: {:#?}", tm);
}
