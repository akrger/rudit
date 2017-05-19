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

        stdout.flush().unwrap();

        for c in std::io::stdin().keys() {
            match c.unwrap() {
                Key::Char('\n') => {
                    buffer.insert(index, '\n');
                    cx = 3;
                    cy += 1;
                    index += 1;
                }
                Key::Char(c) => {
                    buffer.insert(index, c);
                    index += 1;
                    cx += 1;
                }
                Key::Up => {
                    let mut new_line_pos = 0;
                    for i in (0..index).rev() {
                        if buffer.buffer[i] == '\n' {
                            new_line_pos = i;
                            break;
                        }
                    }
                    let mut prev_line = new_line_pos;
                    for i in (0..new_line_pos).rev() {
                        if buffer.buffer[i] == '\n' {
                            break;
                        }
                        prev_line -= 1;
                    }
                    let mut count_previous_line = 0;
                    if new_line_pos == 0 {
                        new_line_pos += 1;
                        count_previous_line = 1;
                    } else {
                        count_previous_line += 1;
                    }
                    for i in (0..new_line_pos).rev() {
                        count_previous_line += 1;
                        if buffer.buffer[i] == '\n' {
                            count_previous_line -= 1;
                            break;
                        }

                    }

                    index = prev_line + std::cmp::min((cx as usize - 3), count_previous_line - 1);
                    if cx - 3 >= count_previous_line as u16 {
                        cx = count_previous_line as u16 + 2;
                    }
                    // println!("");
                    // println!("size {} prevline {}", count_previous_line, prev_line);
                    cy -= 1;
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
