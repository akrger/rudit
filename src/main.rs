extern crate rudit;
extern crate termion;

use std::io::{Write, stdout, Stdout};

use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;
use termion::cursor::Goto;
use termion::raw::RawTerminal;
use rudit::textbuffer::TextBuffer;

fn main() {
    #![allow(warnings)]
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut index = 0;
    let mut cx: u16 = 3;
    let mut cy: u16 = 1;
    let mut line_num = 1;
    let mut size = 0;

    let mut file_opened = false;
    // file_opened = true;
    if file_opened {
        size = file_char_count();
    }

    let mut buffer: TextBuffer = TextBuffer::new();

    if !file_opened {
        //let len = buffer.buf.len() - 1;
        //      buffer.buf[len] = '\n';
        //        buffer.end -= 1;
    }
    if file_opened {
        line_num = open_file(&mut buffer, line_num);
    }
    // assert_eq!(buffer.buffer.len(), 3);
    let (mut line_size, mut start, mut end) = (0, 0, 0);
    write!(stdout,
           "{}{}",
           termion::clear::All,
           termion::cursor::Goto(cx, cy))
            .unwrap();
    stdout.flush().unwrap();
    'main: loop {
        //write!(stdout, "{}", termion::clear::All).unwrap();
        //println!("{:?}", buffer.buf);
        draw_lines(&mut stdout, buffer.buf.clone());
        draw_info(&mut stdout, index, line_num, cx, cy, line_size, start, end);
        draw_cursor(&mut stdout, cx, cy);
        stdout.flush().unwrap();

        for c in std::io::stdin().keys() {
            match c.unwrap() {
                Key::Char('\n') => {
                    buffer.buf[cy as usize - 1].insert(cx as usize - 3, '\n');
                    buffer.buf.insert(cy as usize, vec![]);
                    cx = 3;
                    cy += 1;
                    index = 0;
                    line_num += 1;
                }
                Key::Char(c) => {
                    buffer.buf[cy as usize - 1].insert(index, c);
                    index += 1;
                    cx += 1;
                }
                Key::Backspace => {
                    // buffer.delete(index);
                    if index > 0 {
                        cx -= 1;
                        index -= 1;
                    }
                }
                Key::Up => {
                    if cy > 1 {
                        if buffer.line_size(cy as usize - 1) > buffer.line_size(cy as usize - 2) {
                            cx = buffer.line_size(cy as usize - 2) as u16 + 3;
                            index = cx as usize - 3;
                        }
                        cy -= 1;
                    }
                }
                Key::Down => {
                    if cy < line_num as u16 {
                        if buffer.line_size(cy as usize) < buffer.line_size(cy as usize - 1) {
                            cx = buffer.line_size(cy as usize) as u16 + 3;
                            index = cx as usize - 3;
                        }
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
                    //   if cx - 2 < buffer.count() as u16 {
                    //     cx += 1;
                    //    index += 1;
                    // }
                    if cx - 3 < buffer.line_size(cy as usize - 1) as u16 {
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
fn draw_info(stdout: &mut RawTerminal<Stdout>,
             index: usize,
             line_num: usize,
             cx: u16,
             cy: u16,
             line_count: usize,
             start: usize,
             end: usize) {
    write!(stdout,
           "{} index {} line_num {} cx {} cy {} line_count {} start   {} end   {}",
           termion::cursor::Goto(0, termion::terminal_size().unwrap().1),
           index,
           line_num,
           cx - 2,
           cy,
           line_count,
           start,
           end,
           )
            .unwrap();
}
fn draw_lines(stdout: &mut RawTerminal<Stdout>, buffer: Vec<Vec<char>>) {
    let mut s: String = String::from("");
    for i in buffer {
        for j in i {
            s.push(j);
        }
    }
    // don't draw eof line
    // if s.ends_with('\n') {
    //     s.pop();
    // }
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
fn file_char_count() -> usize {
    use std::io::BufReader;
    use std::fs::File;
    use std::io::prelude::*;
    let mut string = String::from("");
    BufReader::new(File::open("/home/andre/test2").unwrap()).read_to_string(&mut string).unwrap();
    string.len()
}
fn open_file(buffer: &mut TextBuffer, mut line_num: usize) -> usize {
    use std::io::BufReader;
    use std::fs::File;
    use std::io::prelude::*;
    let mut string = String::from("");
    BufReader::new(File::open("/home/andre/test2").unwrap()).read_to_string(&mut string).unwrap();
    for (index, c) in string.chars().enumerate() {
        if c == '\n' {
            line_num += 1;
        }
        // buffer.insert(index, c);
    }

    line_num
}
