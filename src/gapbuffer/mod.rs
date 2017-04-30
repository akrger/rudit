use std::fmt::Display;
use std::fmt::{Formatter, Result};

pub struct GapBuffer {
    pub buffer: Vec<char>,
    cursor: usize,
    pub gap_start: usize,
    gap_end: usize,
}

impl GapBuffer {
    fn check_gap(&mut self) {
        if self.gap_start == self.gap_end {
            let len = self.buffer.clone().len();
            self.buffer.resize(len * 2, ' ')
        }
    }

    pub fn insert(&mut self, ch: char) {
        // verursacht, dass der cursor zu weit
        // vorspringt
        //self.check_gap();
        self.buffer[self.cursor] = ch;
        self.gap_start += 1
    }
    pub fn delete(&mut self) {
        self.buffer[self.cursor] = ' ';
        self.gap_start -= 1
    }
    pub fn new() -> GapBuffer {
        GapBuffer {
            buffer: vec!['\0'; 10],
            gap_start: 0,
            gap_end: 9,
            cursor: 0,
        }
    }
}
impl Display for GapBuffer {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut string = String::new();
        for i in self.buffer.clone() {
            string.push(i);
        }
        write!(f, "{}", string)
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
