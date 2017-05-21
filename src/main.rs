extern crate rudit;
extern crate termion;

use std::io::{Write, stdout, Stdout};

use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;
use termion::cursor::Goto;
use termion::raw::RawTerminal;
use rudit::gapbuffer::GapBuffer;

fn main() {
    let mut buffer = GapBuffer::new(30);
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut index = 0;
    let mut cx: u16 = 3;
    let mut cy: u16 = 1;
    let mut line_num = 1;
    let mut line_size: usize = buffer.get_line_size(cy as usize);
    let mut line: usize = buffer.line_index_to_char_index(cy as usize);

    write!(stdout,
           "{}{}",
           termion::clear::All,
           termion::cursor::Goto(cx, cy))
        .unwrap();
    stdout.flush().unwrap();
    'main: loop {
        write!(stdout, "{}", termion::clear::All).unwrap();

        draw_lines(&mut stdout, &buffer.buffer);
        draw_info(&mut stdout, index, line_num, cx, cy, line_size, line);
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
                    line_size = buffer.get_line_size(cy as usize);
                    line = buffer.line_index_to_char_index(cy as usize);
                }
                Key::Char(c) => {
                    buffer.insert(index, c);
                    index += 1;
                    cx += 1;
                    line_size = buffer.get_line_size(cy as usize);
                    line = buffer.line_index_to_char_index(cy as usize);
                }
                Key::Up => {
                    if cy > 1 {
                        line_size = buffer.get_line_size(cy as usize - 1);
                        line = buffer.line_index_to_char_index(cy as usize - 1);
                        index = line + std::cmp::min((cx as usize - 3), line_size - 1);
                        if line_size < buffer.get_line_size(cy as usize) {
                            cx = line_size as u16 + 2;
                        }
                        cy -= 1;
                    }
                }
                Key::Down => {
                    if cy < line_num as u16 {
                        line_size = buffer.get_line_size(cy as usize + 1);
                        line = buffer.line_index_to_char_index(cy as usize + 1);
                        index = line + std::cmp::min((cx as usize - 3), line_size - 1);
                        cy += 1;
                        if cx > line_size as u16 {
                            cx = line_size as u16 + 3;
                        }
                    }
                }
                Key::Left => {
                    if cx > 3 {
                        cx -= 1;
                        index -= 1;
                        line_size = buffer.get_line_size(cy as usize);
                        line = buffer.line_index_to_char_index(cy as usize);
                    }
                }
                Key::Right => {
                    // not working correctly
                    if cx - 3 < buffer.get_line_size(cy as usize) as u16 - 1 {
                        cx += 1;
                        index += 1;
                        line_size = buffer.get_line_size(cy as usize);
                        line = buffer.line_index_to_char_index(cy as usize);
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
fn draw_info(stdout: &mut RawTerminal<Stdout>,
             index: usize,
             line_num: usize,
             cx: u16,
             cy: u16,
             line_size: usize,
             line: usize) {
    write!(stdout,
           "{} index {} line_num {} cx {} cy {} line_size {} line {}",
           termion::cursor::Goto(0, termion::terminal_size().unwrap().1),
           index,
           line_num,
           cx - 2,
           cy,
           line_size,
           line)
        .unwrap();
}
fn draw_lines(stdout: &mut RawTerminal<Stdout>, buffer: &Vec<char>) {
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

fn draw_cursor(stdout: &mut RawTerminal<Stdout>, cx: u16, cy: u16) {
    write!(stdout, "{}", termion::cursor::Goto(cx, cy)).unwrap();
}
