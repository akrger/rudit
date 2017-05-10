use std::fmt::Display;
use std::fmt::{Formatter, Result};

pub struct GapBuffer {
    pub buffer: Vec<char>,
    pub cursor: usize,
    pub gap_start: usize,
    pub gap_end: usize,
}

impl GapBuffer {
    fn place_gap(&mut self) {
        if self.cursor == self.gap_start {
            return;
        }
        if self.cursor < self.gap_start {
            let count = self.gap_end - self.gap_start;
            let amt = self.gap_start - self.cursor;

            for i in self.cursor..self.gap_start {
                self.buffer[i + count] = self.buffer[i];
            }
            self.gap_start -= amt;
            self.gap_end -= amt;

            for i in self.gap_start..self.gap_end {
                self.buffer[i] = '\0';
            }
        } else {
            let count = self.gap_end - self.gap_start;
            let amt = self.cursor - self.gap_start;

            for i in self.gap_end..self.buffer.len() {
                self.buffer[i - count] = self.buffer[i];
            }
            self.gap_start += amt;
            self.gap_end += amt;

            for i in self.gap_start..self.gap_end {
                self.buffer[i] = '\0';
            }

        }
    }
    fn check_capacity(&mut self) {
        if self.gap_end == self.gap_start {
            self.extend_buffer();
        }
    }
    fn extend_buffer(&mut self) {
        let len = self.buffer.clone().len();
        self.buffer.resize(len * 2, '\0');
        // still buggy :(
        // for i in self.gap_end..self.buffer.len() - len {
        //     self.buffer[i + len] = self.buffer[i];
        // }
        // self.gap_end += len;

        // for i in self.gap_start..self.gap_end {
        //     self.buffer[i] = '\0';
        // }
    }
    pub fn insert(&mut self, ch: char) {
        self.check_capacity();
        self.place_gap();
        self.buffer[self.cursor] = ch;
        self.cursor += 1;
        self.gap_start += 1;
    }
    pub fn delete(&mut self) {
        if self.gap_start > 0 && self.cursor > 0 {
            self.cursor -= 1;
            self.gap_start -= 1;
        }
        for i in self.gap_start..self.gap_end {
            self.buffer[i] = '\0';
        }
    }
    pub fn new(size: usize) -> GapBuffer {
        GapBuffer {
            buffer: vec!['\0'; size],
            gap_start: 0,
            gap_end: size,
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
    fn test_gap_check() {
        use gapbuffer::GapBuffer;
        let mut buffer = GapBuffer::new(5);
        buffer.insert('a');
        buffer.insert('c');

        assert_eq!('a', buffer.buffer[0]);
        assert_eq!('c', buffer.buffer[1]);
        buffer.cursor = 0;
        buffer.insert('d');
        assert_eq!('d', buffer.buffer[0]);
        assert_eq!('a', buffer.buffer[3]);
        assert_eq!('c', buffer.buffer[4]);
    }
    //#[test]
    fn test_gap_check_1() {
        use gapbuffer::GapBuffer;
        let mut buffer = GapBuffer::new(5);
        assert_eq!(4, buffer.gap_end - buffer.gap_start);
        buffer.insert('a');
        assert_eq!(3, buffer.gap_end - buffer.gap_start);
        buffer.cursor = 0;
        buffer.insert('b');
        assert_eq!(2, buffer.gap_end - buffer.gap_start);
        buffer.insert('b');
        assert_eq!(1, buffer.gap_end - buffer.gap_start);
        buffer.insert('b');
        assert_eq!(0, buffer.gap_end - buffer.gap_start);
        buffer.insert('b');

        assert_eq!(4, buffer.gap_end - buffer.gap_start);
    }
    #[test]
    fn test_gap_check_2() {
        use gapbuffer::GapBuffer;
        let mut buffer = GapBuffer::new(5);
        buffer.insert('a');
        buffer.cursor = 3;
        buffer.insert('b');
        assert_eq!('a', buffer.buffer[0]);
        assert_eq!('b', buffer.buffer[3]);
    }
    #[test]
    fn test_gap_check_3() {
        use gapbuffer::GapBuffer;
        let mut buffer = GapBuffer::new(5);
        buffer.insert('a');
        buffer.insert('b');
        buffer.insert('c');
        buffer.insert('d');
        assert_eq!('a', buffer.buffer[0]);
        assert_eq!('b', buffer.buffer[1]);
        assert_eq!('c', buffer.buffer[2]);
        assert_eq!('d', buffer.buffer[3]);

        buffer.cursor = 2;
        buffer.insert(' ');
        assert_eq!(' ', buffer.buffer[2]);
        assert_eq!('c', buffer.buffer[3]);
    }
    #[test]
    fn test_gap_check_4() {
        use gapbuffer::GapBuffer;
        let mut buffer = GapBuffer::new(5);
        buffer.insert('a');
        buffer.insert('b');
        assert_eq!('a', buffer.buffer[0]);
        assert_eq!('b', buffer.buffer[1]);
        buffer.cursor = 0;
        buffer.insert(' ');
        assert_eq!(' ', buffer.buffer[0]);
        assert_eq!('a', buffer.buffer[3]);
        assert_eq!('b', buffer.buffer[4]);
        assert_eq!(1, buffer.gap_start);
        assert_eq!(4, buffer.gap_end);
    }

}
