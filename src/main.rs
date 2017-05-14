extern crate rudit;
extern crate termion;

use std::io::{Write, stdout};
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;
use rudit::gapbuffer::GapBuffer;

fn main() {
    let mut buffer = GapBuffer::new(30);
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut index = 0;
    let mut cx = 1;
    let mut cy = 1;
    write!(stdout,
           "{}{}",
           termion::clear::All,
           termion::cursor::Goto(cx as u16, cy as u16))
        .unwrap();
    stdout.flush().unwrap();
    loop {
        for c in std::io::stdin().keys() {
            match c.unwrap() {
                Key::Char('\n') => {
                    cx = 1;
                    cy += 1;
                }
                Key::Char(c) => {
                    buffer.insert(index, c);
                    index += 1;
                    cx += 1;
                }
                Key::Left => {
                    if cx > 0 {
                        cx -= 1
                    }
                }
                Key::Right => {
                    if cx <= buffer.len() {
                        cx += 1
                    }
                }
                Key::Esc => break,
                _ => (),
            }
            // move cursor
            write!(stdout, "{}", termion::cursor::Goto(cx as u16, cy as u16)).unwrap();
            stdout.flush().unwrap();
        }
        break;
    }
}
