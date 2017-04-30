extern crate termion;
extern crate rudit;

use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;
use std::io::{Write, stdin, stdout};

use rudit::gapbuffer::GapBuffer;

fn main() {
    let mut buffer = GapBuffer::new();
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout,
           "{}{}",
           termion::cursor::Goto(1, 1),
           termion::clear::All)
            .unwrap();
    stdout.flush().unwrap();
    for c in stdin.keys() {
        stdout.flush().unwrap();
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char('\n') => {
                buffer.insert('\n');
                write!(stdout, "{}", termion::cursor::Down(1)).unwrap();
                write!(stdout, "{}", termion::cursor::Left(buffer.gap_start as u16)).unwrap();
                stdout.flush().unwrap();
                ()
            }
            Key::Char(c) => {
                buffer.insert(c);
                write!(stdout, "{}", buffer).unwrap();
                stdout.flush().unwrap();
            }
            Key::Up => {
                write!(stdout, "{}", termion::cursor::Up(1)).unwrap();
                stdout.flush().unwrap();
                ()
            }
            Key::Left => {
                write!(stdout, "{}", termion::cursor::Left(1)).unwrap();
                stdout.flush().unwrap();
                ()
            }
            Key::Right => {
                write!(stdout, "{}", termion::cursor::Right(1)).unwrap();
                stdout.flush().unwrap();
                ();
            }
            _ => (),
        }
    }
    write!(stdout, "{}", termion::clear::All).unwrap();
    stdout.flush().unwrap();
}
