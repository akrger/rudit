extern crate gapbuffer;
extern crate termion;
use std::io::{Write, stdout, Stdout};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::event::Key;
use termion::input::TermRead;
use termion::cursor::Goto;
use gapbuffer::GapBuffer;

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut cx: u16 = 3;
    let mut cy: u16 = 1;
    let mut buffer = GapBuffer::new(5);
    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(cx, cy)).unwrap();
    stdout.flush().unwrap();
    'main: loop {
        write!(stdout, "{}", termion::clear::All).unwrap();
        let line_len = get_line_len(&buffer);
        draw_buffer(&mut stdout, &buffer.buffer);
        draw_info(&mut stdout, line_len, buffer.point as u16);
        update_cursor(&mut stdout, cx, cy);
        stdout.flush().unwrap();
        for c in std::io::stdin().keys() {
            match c.unwrap() {

                Key::Char('\n') => {
                    buffer.insert('\n');
                    buffer.point+=1;
                    cy += 1;
                    cx = 3;
                },
                Key::Char(c) => {
                    buffer.insert(c);
                    buffer.point+=1;
                    cx += 1;
                },
                
                Key::Up => {
                    
                }
                Key::Down => {
                    
                }
                Key::Left => {
                    if cx >= 4 {                        
                        cx -= 1;
                        buffer.point -= 1 
                    }
                },
                Key::Right => {
                    if cx <= line_len +2    {
                        cx += 1 ;
                        buffer.point += 1
                    }
                },
                Key::Home => {
                    buffer.place_gap()
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
fn update_cursor(stdout: &mut RawTerminal<Stdout>, cx: u16, cy: u16) {
    write!(stdout, "{}", termion::cursor::Goto(cx, cy)).unwrap()
}
fn draw_buffer(stdout: &mut RawTerminal<Stdout>, buffer: &Vec<char>) {
    let mut lines: u16 = 1;
    let mut line_start: u16 = 3;
    for ch in buffer {
        if *ch == '\n' {
            lines += 1;
            line_start = 2
        }
        if *ch != '\0' {
            write!(stdout, "{}{}", Goto(line_start, lines), ch).unwrap();
            line_start += 1
        }
    }
}
fn get_line_len(buffer: &GapBuffer) -> u16 {
    let mut line_len = 0;
    let mut line_start = 0;
    for i in 0..buffer.point  {
        if buffer.buffer[i] == '\n' {
            line_start = i
        }
    }
    for i in line_start..buffer.buffer.len() {
        if buffer.buffer[i] == '\n' {
            continue
        }
        if buffer.buffer[i] != '\0' {
            line_len += 1;
        }
    }
    line_len
}
fn draw_info(stdout: &mut RawTerminal<Stdout>, line_len: u16, point: u16) {
    write!(stdout, "{}{} --- {}", Goto(3, 20), line_len, point).unwrap();
}
