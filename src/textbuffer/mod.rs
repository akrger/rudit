pub struct TextBuffer {
    pub buf: Vec<Vec<char>>,
}

impl TextBuffer {
    pub fn new() -> TextBuffer {
        TextBuffer {buf: vec![vec!['\0'; 0];1]}
    }
    pub fn line_size(&self, line_num: usize) -> usize {
        if self.buf[line_num].last() == Some(&'\n') {
            self.buf[line_num].len() - 1
        }
        else {
            self.buf[line_num].len()
        }
    }
}
#[cfg(test)]
mod tests {}
