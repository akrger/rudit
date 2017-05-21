extern crate rudit;
extern crate termion;

use std::io::{Write, stdout};
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;
use termion::cursor::Goto;
use rudit::gapbuffer::GapBuffer;
use termion::cursor::DetectCursorPos;

fn main() {
    let mut buffer = GapBuffer::new(30);
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut index = 0;
    let mut line_num = 1;
    let mut cx: u16 = 3;
    let mut cy: u16 = 1;
    write!(stdout,
           "{}{}",
           termion::clear::All,
           termion::cursor::Goto(cx, cy))
        .unwrap();
    stdout.flush().unwrap();
    'main: loop {
        write!(stdout, "{}", termion::clear::All).unwrap();

        draw_lines(&mut stdout, &buffer.buffer);
        draw_info(&mut stdout, index, line_num, cx, cy);
        draw_cursor(&mut stdout, cx, cy);
        stdout.flush().unwrap();

        for c in std::io::stdin().keys() {
            match c.unwrap() {
                Key::Char('\n') => {
                    buffer.insert(index, '\n');
                    cx = 3;
                    cy += 1;
                    index += 1;
                    line_num += 1;
                }
                Key::Char(c) => {
                    buffer.insert(index, c);
                    index += 1;
                    cx += 1;
                }
                Key::Up => {
                    if cy > 1 {
                        cy -= 1;
                    }
                }
                Key::Down => {
                    if cy < line_num as u16 {
                        cy += 1;
                    }
                }
                Key::Left => {
                    if cx > 3 {
                        cx -= 1;
                        index -= 1;
                    }
                }
                Key::Right => {
                    if buffer.buffer[index] != '\0' ||
                       buffer.buffer[buffer.buffer.len() - index + buffer.gap_start - 1] != '\0' {
                        cx += 1;
                        index += 1;
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
fn draw_info(stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
             index: usize,
             line_num: usize,
             cx: u16,
             cy: u16) {
    write!(stdout,
           "{} index {} line_num {} cx {} cy {}",
           termion::cursor::Goto(0, termion::terminal_size().unwrap().1),
           index,
           line_num,
           cx - 2,
           cy)
        .unwrap();
}
fn draw_lines(stdout: &mut termion::raw::RawTerminal<std::io::Stdout>, buffer: &Vec<char>) {
    let s: String = buffer.iter().collect();
    for (index, i) in s.split('\n').enumerate() {
        write!(stdout,
               "{}{}{}{}",
               Goto(0, (index + 1) as u16),
               index + 1,
               Goto(3, (index + 1) as u16),
               i)
            .unwrap();
    }
}

fn draw_cursor(stdout: &mut termion::raw::RawTerminal<std::io::Stdout>, cx: u16, cy: u16) {
    write!(stdout, "{}", termion::cursor::Goto(cx, cy)).unwrap();
}
