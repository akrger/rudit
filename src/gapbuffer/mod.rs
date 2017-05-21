use std::fmt::Display;
use std::fmt::{Formatter, Result};

pub struct GapBuffer {
    pub buffer: Vec<char>,
    pub gap_start: usize,
    pub gap_end: usize,
}

impl GapBuffer {
    fn place_gap(&mut self, index: usize) {
        if index == self.gap_start {
            return;
        }
        if index < self.gap_start {
            let count = self.gap_end - self.gap_start;
            let amt = self.gap_start - index;

            let temp_buffer = self.buffer.clone();
            for i in index..self.gap_start {
                self.buffer[i + count] = temp_buffer[i];
            }
            self.gap_start -= amt;
            self.gap_end -= amt;

            for i in self.gap_start..self.gap_end {
                self.buffer[i] = '\0';
            }
        } else {
            let count = self.gap_end - self.gap_start;
            let amt = index - self.gap_start;

            let temp_buffer = self.buffer.clone();
            for i in self.gap_end..self.buffer.len() {
                self.buffer[i - count] = temp_buffer[i];
            }
            self.gap_start += amt;
            self.gap_end += amt;

            for i in self.gap_start..self.gap_end {
                self.buffer[i] = '\0';
            }
        }
    }
    fn check_capacity(&mut self, index: usize) {
        if self.gap_end == self.gap_start {
            self.extend_buffer(index);
        }
    }
    fn extend_buffer(&mut self, index: usize) {
        let len = self.buffer.clone().len();
        self.buffer.resize(len * 2, '\0');
        self.gap_start += len - index;
        self.gap_end = self.buffer.len();
        self.place_gap(index);
    }
    pub fn insert(&mut self, index: usize, ch: char) {
        self.place_gap(index);
        self.check_capacity(index);
        self.buffer[index] = ch;
        self.gap_start += 1;
    }
    pub fn delete(&mut self, index: usize) {
        self.place_gap(index);
        self.buffer[self.gap_end] = '\0';
        self.gap_end += 1;
    }
    pub fn new(size: usize) -> GapBuffer {
        GapBuffer {
            buffer: vec!['\0'; size],
            gap_start: 0,
            gap_end: size,
        }
    }
    pub fn len(&self) -> usize {
        let mut len: usize = 0;
        for i in self.buffer.clone() {
            if i != '\0' {
                len += 1
            }
        }
        len
    }
    pub fn get_line_size(&self, line_num: usize) -> usize {
        let mut line_start = 0;
        let mut line_end = 0;
        let mut eol_count = 0;

        if line_num == 1 {
            for i in 0..self.buffer.len() {
                if self.buffer[i] == '\n' {
                    break;
                }
                // don't count empty char
                if self.buffer[i] == '\0' {
                    continue;
                }
                eol_count += 1;
            }
            for i in eol_count..self.buffer.len() {
                if self.buffer[i] == '\n' {
                    // newline counts, too
                    eol_count = i + 1;
                    break;
                }
                // if gap moved
                if self.buffer[i] == '\0' {
                    // eol needs to be accounted for
                    eol_count += 1;
                    break;
                }
            }
            return self.buffer[line_start..eol_count].len();
        } else {
            return self.buffer[line_start..line_end].len();
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
        buffer.insert(0, 'a');
        buffer.insert(1, 'c');

        assert_eq!('a', buffer.buffer[0]);
        assert_eq!('c', buffer.buffer[1]);
        buffer.insert(0, 'd');
        assert_eq!('d', buffer.buffer[0]);
        assert_eq!('a', buffer.buffer[3]);
        assert_eq!('c', buffer.buffer[4]);
    }
}
