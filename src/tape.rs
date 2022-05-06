use crate::tuple::Moves;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug, PartialEq)]
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

    pub fn move_head(&mut self, dir: Moves) {
        match dir {
            Moves::L => self.move_left(),
            Moves::R => self.move_right(),
            Moves::S => {}
        }
    }

    fn move_right(&mut self) {
        if self.pos as usize == self.content.len() - 1 {
            self.content.push(self.white_char);
        }
        self.pos += 1;
    }

    fn move_left(&mut self) {
        if self.pos == 0 {
            let mut new_content: Vec<char> = vec![self.white_char; 1];
            new_content.append(&mut self.content);
            self.content = new_content;
        }
        self.pos -= 1;
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

    pub fn get_white_char(&self) -> char {
        self.white_char
    }

    pub fn get_content(&self) -> Vec<char> {
        self.content.clone()
    }

    pub fn load_from_file(filename: String) -> Result<Tape, io::Error> {
        let file = File::open(filename)?;
        let file_reader = BufReader::new(file);
        let data: String = file_reader.lines().collect::<Result<_, _>>().unwrap();
        let default_white: char = '$';
        let tape: Tape = Tape::new(data.chars().collect(), default_white);
        Ok(tape)
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn tape_load_from_file() {
        let filename: String = String::from("notfound.tape");
        assert_eq!(Tape::load_from_file(filename).is_err(), true);
        let filename: String = String::from("tapes/example1.tape");
        assert_eq!(Tape::load_from_file(filename).is_err(), false);
        let sample_tape: Tape = Tape::new("aaaabbbb".chars().collect(), '$');
        let tape: Tape =
            Tape::load_from_file(String::from("tapes/example1.tape")).expect("Error creating Tape");
        assert_eq!(sample_tape, tape);
    }

    #[test]
    fn tape_methods() {
        let mut tape: Tape =
            Tape::load_from_file(String::from("tapes/example1.tape")).expect("Error creating Tape");
        assert_eq!(0, tape.get_pos());
        let new_char: char = '%';
        tape.set_white_char(new_char);
        assert_eq!(new_char, tape.get_white_char());
        assert_eq!('a', tape.get_char_at_pos());
        tape.set_char_at_pos(new_char);
        assert_eq!(new_char, tape.get_char_at_pos());
        // Nos movemos a la derecha
        tape.move_right();
        assert_eq!(tape.get_char_at_pos(), 'a');
        // Nos movemos a la izquierda
        tape.move_left();
        assert_eq!(tape.get_char_at_pos(), '%');
    }
}
