use std::fmt::Display;
use std::fmt::{Formatter, Result};

pub struct GapBuffer {
    pub buffer: Vec<char>,
    pub cursor: usize,
    pub gap_start: usize,
    gap_end: usize,
}

impl GapBuffer {
    fn check_gap(&mut self) {
        // buffer voll
        if self.gap_end == self.gap_start {
            let len = self.buffer.clone().len();
            self.buffer.resize(len * 2, ' ');
            self.gap_end = self.buffer.len();
        }
        // links vom cursor

        // rechts vom cursor
    }

    pub fn insert(&mut self, index: usize, ch: char) {
        // verursacht, dass der cursor zu weit
        // vorspringt
        self.check_gap();
        self.buffer[self.gap_start] = ch;
        self.gap_start += 1
    }
    pub fn delete(&mut self) {
        self.buffer[self.cursor] = '\0';
        self.gap_start -= 1
    }
    pub fn new(size: usize) -> GapBuffer {
        GapBuffer {
            buffer: vec!['\0'; size],
            gap_start: 0,
            gap_end: size - 1,
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
    fn test_insert() {
        use gapbuffer::GapBuffer;
        let mut buffer = GapBuffer::new(5);
        buffer.insert(0, 'a');
        assert_eq!('a', buffer.buffer[0]);
    }
    #[test]
    fn test_insert2() {
        use gapbuffer::GapBuffer;
        let mut buffer = GapBuffer::new(2);
        buffer.insert(0, 'a');
        assert_eq!(2, buffer.buffer.len());
        buffer.insert(1, 'c');
        buffer.insert(2, 'b');
        assert_eq!('a', buffer.buffer[0]);
        assert_eq!('b', buffer.buffer[2]);
        assert_eq!(4, buffer.buffer.len());
        assert_eq!(3, buffer.gap_start);
        assert_eq!(4, buffer.gap_end);
    }
    #[test]
    fn test_insert3() {
        use gapbuffer::GapBuffer;
        let mut buffer = GapBuffer::new(5);
        buffer.insert(0, 't');
        buffer.insert(1, 'h');
        buffer.insert(2, 'e');
        assert_eq!(1, buffer.gap_end - buffer.gap_start);
        assert_eq!(3, buffer.gap_start);
        assert_eq!(4, buffer.gap_end);
        assert_eq!(5, buffer.buffer.len());
        buffer.insert(3, ' ');
        buffer.insert(4, 'f');
        assert_eq!(10, buffer.buffer.len());
        buffer.insert(5, 'o');
        buffer.insert(6, 'x');
        // insert before f
        buffer.insert(4, 'w');
        buffer.insert(5, 'h');
        buffer.insert(6, 'i');
        buffer.insert(7, 't');
        buffer.insert(8, 'e');
        assert_eq!('w', buffer.buffer[4]);
        assert_eq!('f', buffer.buffer[9]);
    }
}
