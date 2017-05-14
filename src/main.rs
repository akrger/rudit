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
    let mut cx: u16 = 1;
    let mut cy: u16 = 1;
    write!(stdout,
           "{}{}",
           termion::clear::All,
           termion::cursor::Goto(cx, cy))
        .unwrap();
    stdout.flush().unwrap();
    'main: loop {

        write!(stdout, "{}", termion::clear::All).unwrap();
        stdout.flush().unwrap();

        draw_lines(&mut stdout);
        draw_cursor(&mut stdout, cx, cy);

        for c in std::io::stdin().keys() {
            match c.unwrap() {
                Key::Char('\n') => {
                    buffer.insert(index, '\n');
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
                    if cx as usize <= buffer.len() {
                        cx += 1
                    }
                }
                Key::Esc => break,
                _ => (),
            }
            continue 'main;
        }
        write!(stdout, "{}", termion::clear::All).unwrap();
        stdout.flush().unwrap();
        break;
    }
}
fn draw_lines(stdout: &mut termion::raw::RawTerminal<std::io::Stdout>) {
    // TODO
    // implement real drawing of buffer lines and line numbers
    for i in 0..5 {
        write!(stdout,
               "{}{}",
               termion::cursor::Goto(0, i + 1),
               (i + 1).to_string())
            .unwrap();
    }
    stdout.flush().unwrap();
}

fn draw_cursor(stdout: &mut termion::raw::RawTerminal<std::io::Stdout>, cx: u16, cy: u16) {
    write!(stdout, "{}", termion::cursor::Goto(cx, cy)).unwrap();
    stdout.flush().unwrap();
}
