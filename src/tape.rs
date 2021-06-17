use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
pub struct Tape {
    content:  Vec<char>,
    pos:  u32,
}

impl Tape {
    pub fn new(content: Vec<char>, pos: u32) -> Tape {
        Tape { content, pos }
    }

    pub fn get_pos(&self) -> u32 { self.pos }

    pub fn update_pos(&mut self, new_pos: u32) {
        if new_pos >= self.content.len() as u32 {
            panic!("Error while updating the position in the tape with the value: {}", new_pos);
        }
        self.pos = new_pos
    }

    pub fn set_char_at_pos(&mut self, new_char: char) {
        self.content[self.pos as usize] = new_char
    }

    pub fn get_char_at_pos(&self) -> char {
        self.content[self.pos as usize]
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
