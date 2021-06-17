use itertools::Itertools;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
pub struct Tape {
    content: Vec<char>,
    pos: u32,
}

impl Tape {
    pub fn new(content: Vec<char>, pos: u32) -> Tape {
        Tape { content, pos }
    }
}

pub fn load_from_file(filename: String) -> Result<Tape, io::Error> {
    let file = File::open(filename)?;
    let file_reader = BufReader::new(file);
    let data: String = file_reader.lines().collect::<Result<_, _>>().unwrap();
    let tape: Tape = Tape::new(
        data.chars().collect(),
        0
    );
    Ok(tape)
}
