pub struct GapBuffer {
    buffer: Vec<char>,
    cursor: usize,
    gap_start: usize,
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
        self.check_gap();
        self.buffer[self.cursor] = ch;
        self.gap_start += 1
    }
    pub fn delete(&mut self) {
        self.buffer[self.cursor] = ' ';
        self.gap_start -= 1
    }
    pub fn new() -> GapBuffer {
        GapBuffer {
            buffer: Vec::with_capacity(10),
            gap_start: 0,
            gap_end: 9,
            cursor: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
