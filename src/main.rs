extern crate termion;

use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;
use termion::cursor::DetectCursorPos;
use std::io::{Write, stdin, stdout};

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let (tx, ty) = termion::terminal_size().unwrap();
    write!(stdout,
           "{}{}",
           termion::cursor::Goto(1, 1),
           termion::clear::All)
        .unwrap();
    stdout.flush().unwrap();
    let (mut _cx, mut _cy) = (0, 0);
    let mut cy = 1;
    let mut cx = 1;
    for c in stdin.keys() {
        stdout.flush().unwrap();
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char('\n') => {
                cy += 1;
                write!(stdout, "{}", termion::cursor::Goto(1, cy)).unwrap();
                stdout.flush().unwrap();
                ()
            }
            Key::Char(c) => {
                cx += 1;
                write!(stdout, "{}", c).unwrap();
                stdout.flush().unwrap();
            }
            Key::Up => {
                cy -= 1;
                write!(stdout, "{}", termion::cursor::Up(1)).unwrap();
                stdout.flush().unwrap();
                ()
            }
            Key::Left => {
                cx -= 1;
                write!(stdout, "{}", termion::cursor::Left(1)).unwrap();
                stdout.flush().unwrap();
                ()
            }
            Key::Right => {
                cx += 1;
                write!(stdout, "{}", termion::cursor::Right(1)).unwrap();
                stdout.flush().unwrap();
                ();
            }
            Key::Ctrl('s') => {
                _cx = stdout.cursor_pos().unwrap().0;
                _cy = stdout.cursor_pos().unwrap().1;
                write!(stdout, "{}Saved", termion::cursor::Goto(1, ty)).unwrap();
                write!(stdout, "{}", termion::cursor::Goto(_cx, _cy)).unwrap();
                stdout.flush().unwrap();
                ()
            }
            _ => (),
        }
        stdout.flush().unwrap();
    }
    write!(stdout, "{}", termion::clear::All).unwrap();
    println!("{} {}", cx, cy);
    stdout.flush().unwrap();
}
