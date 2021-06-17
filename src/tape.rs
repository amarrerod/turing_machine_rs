use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
pub struct Tape {
    content: Vec<char>,
    pos: u32,
    white_char: char,
}

impl Tape {
    pub fn new(content: Vec<char>, white_char: char) -> Tape {
        Tape {
            content,
            pos: 0,
            white_char,
        }
    }

    pub fn get_pos(&self) -> u32 {
        self.pos
    }

    pub fn update_pos(&mut self, new_pos: i32) {
        if new_pos as usize >= self.content.len() {
            self.content.push(self.white_char);
        } else if new_pos <= 0 {
            let mut new_content: Vec<char> = vec![self.white_char; 1];
            new_content.append(&mut self.content);
            self.content = new_content;
            self.pos = 0;
        } else {
            self.pos = new_pos as u32;
        }
    }

    pub fn set_char_at_pos(&mut self, new_char: char) {
        self.content[self.pos as usize] = new_char;
    }

    pub fn get_char_at_pos(&self) -> char {
        self.content[self.pos as usize]
    }

    pub fn set_white_char(&mut self, white: char) {
        self.white_char = white;
    }
}

pub fn load_from_file(filename: String) -> Result<Tape, io::Error> {
    let file = File::open(filename)?;
    let file_reader = BufReader::new(file);
    let data: String = file_reader.lines().collect::<Result<_, _>>().unwrap();
    let default_white: char = '$';
    let tape: Tape = Tape::new(data.chars().collect(), default_white);
    Ok(tape)
}
