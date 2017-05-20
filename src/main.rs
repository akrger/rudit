extern crate rudit;
extern crate termion;

use std::io::{Write, stdout};
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;
use termion::cursor::Goto;
use rudit::gapbuffer::GapBuffer;

fn main() {
    let mut buffer = GapBuffer::new(30);
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut index = 0;
    let mut line_num = 0;
    let mut cx: u16 = 3;
    let mut cy: u16 = 1;
    // buffer.insert(0, 'a');
    // buffer.insert(1, 'b');
    // buffer.insert(2, '\n');
    // buffer.insert(3, 'c');
    // index = 3;
    // cy = 2;
    write!(stdout,
           "{}{}",
           termion::clear::All,
           termion::cursor::Goto(cx, cy))
        .unwrap();
    stdout.flush().unwrap();
    'main: loop {

        write!(stdout, "{}", termion::clear::All).unwrap();

        draw_lines(&mut stdout, &buffer.buffer);
        draw_cursor(&mut stdout, cx, cy);
        // println!("         ");
        // println!("{}", index);
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
                        let prev_line = buffer.line_index_to_char_index(cy as usize - 1);
                        let prev_line_size = buffer.get_line(cy as usize - 1).len();
                        index = prev_line + std::cmp::min((cx as usize - 3), prev_line_size - 1);
                        if cx - 3 >= prev_line_size as u16 {
                            cx = prev_line_size as u16 + 2;
                        }
                    }
                }
                Key::Down => {
                    if cy <= line_num {
                        cy += 1;
                        let mut eol = index;
                        for i in index..buffer.len() - 1 {
                            if buffer.buffer[i] == '\n' {
                                eol += 1;
                                break;
                            }
                            eol += 1;
                        }
                        let mut size = 0;
                        for i in eol..buffer.len() {
                            if buffer.buffer[i] == '\n' {
                                break;
                            }
                            size += 1;

                        }
                        // println!("");
                        // println!("eol {} size {}", eol, size);
                       // if cx > size {
                       //     cx = size + 3;
                        //}
                        index = eol + 1;
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
