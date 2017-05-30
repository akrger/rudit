use std::fmt::{Display, Formatter, Result};

pub struct GapBuffer {
    pub buf: Vec<char>,
    pub start: usize,
    pub end: usize,
}

impl GapBuffer {
    pub fn with_capacity(capacity: usize) -> GapBuffer {
        GapBuffer {
            buf: vec!['\0'; capacity],
            start: 0,
            end: capacity,
        }
    }
    pub fn insert(&mut self, index: usize, ch: char) {
        self.place_gap(index);
        self.buf[index] = ch;
        self.start += 1;
    }
    pub fn size(&self) -> usize {
        self.end - self.start
    }
    pub fn count(&self) -> usize {
        self.buf.len() - self.size()
    }
    pub fn user_to_gap(&self, index: usize) -> usize {
        // client dafür zuständig
        // assert!(index < self.count());
        if index < self.start {
            index
        } else {
            self.size() + index
        }
    }
    pub fn place_gap(&mut self, index: usize) {
        if index == self.start {
            return;
        }
        if index < self.start {
            let count = self.start - index;
            let amt = self.start - self.user_to_gap(index);
            let temp = self.buf.clone();
            self.start += count;
            self.end += count;
            for i in index..self.start {
                self.buf[i + amt] = temp[i];
            }
            //for i in self.start..self.end {
            //    self.buf[i] == '\0';
            //}
        }
    }
}

impl Display for GapBuffer {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut string = String::new();
        for i in self.buf.clone() {
            if i != '\0' {
                string.push(i);
            }
        }
        write!(f, "{}", string)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_stuff() {
        use gapbuffer::GapBuffer;
        let mut buf = GapBuffer::with_capacity(17);
        buf.insert(0, 'M');
        buf.insert(1, 'i');
        buf.insert(2, 'n');
        buf.insert(3, 'n');
        buf.insert(4, 'e');
        buf.insert(5, 'a');
        buf.insert(6, 'p');
        buf.insert(7, 'o');
        buf.insert(14, 'l');
        buf.insert(15, 'i');
        buf.insert(16, 's');
        buf.start = 8;
        buf.end = 13;
        assert_eq!(buf.user_to_gap(5), 5);
        assert_eq!(buf.user_to_gap(9), 14);
        assert_eq!(buf.user_to_gap(11), 16);
        let _buf = buf.buf.clone();
        buf.insert(4, ' ');
        assert_eq!(_buf,
                   buf.buf,
                   "\n\n le {:?} \n\n ri {:?} \n\n",
                   _buf,
                   buf.buf);
    }
}
