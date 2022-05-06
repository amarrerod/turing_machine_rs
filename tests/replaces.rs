use turing_machine::{TuringMachine, tape, utils};

#[test]
fn test_example_machine() {
    let mut tm: TuringMachine =
        utils::load_from_instance("tests/data/example1.tm".to_string(), "tests/data/example1.tape".to_string())
            .expect("Error loading TM");
    let tape: &tape::Tape = tm.run().unwrap();
    println!("The result is: {:?}", tape);
}
