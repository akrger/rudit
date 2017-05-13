extern crate rudit;
extern crate pancurses;

use pancurses::*;
use rudit::gapbuffer::GapBuffer;

fn main() {
    let mut buffer = GapBuffer::new(4);

    let mut stdscr = initscr();

    stdscr.keypad(true);
    noecho();
    nl();
    cbreak();
    let mut cy = 0;
    loop {
        stdscr.refresh();
        draw_buffer(buffer.buffer.clone(), &mut stdscr);
        draw_info(&mut stdscr, &buffer);
        move_cursor(buffer.cursor as i32, cy, &mut stdscr);

        match stdscr.getch() {
            Some(Input::Character(c)) => {
                if c == '\n' {
                    cy += 1;
                    buffer.cursor = 0;
                    buffer.insert('\n');
                }
                buffer.insert(c);
            }
            Some(Input::KeyEnter) => {
                cy += 1;
                buffer.cursor = 0;
                buffer.insert('\n');
            }
            Some(Input::KeyRight) => {
                if buffer.cursor < buffer.len() {
                    buffer.cursor += 1;
                }
            }
            Some(Input::KeyLeft) => {
                if buffer.cursor > 0 {
                    buffer.cursor -= 1;
                }
            }
            Some(Input::KeyDC) => {
                if buffer.cursor > 0 {
                    buffer.cursor -= 1;
                    buffer.delete();
                    stdscr.clear();
                }
            }

            _ => (),
        }
    }
}
fn draw_buffer(buffer: Vec<char>, stdscr: &mut Window) {
    let mut x = 0;
    let mut y = 0;
    for c in buffer {
        if c == '\n' {
            y += 1;
            x = 0;

        } else if c == '\r' || c == '\0' {

        } else {
            stdscr.mvaddch(y, x, c);
            x += 1;
        }
    }
}
fn move_cursor(x: i32, y: i32, stdscr: &mut Window) {
    stdscr.mv(y, x);
}
fn draw_info(stdscr: &mut Window, buffer: &GapBuffer) {
    stdscr.mvaddstr(10, 0, "cx");
    stdscr.mvaddstr(10, 3, buffer.cursor.to_string().as_str());

    stdscr.mvaddstr(10, 5, "gs");
    stdscr.mvaddstr(10, 8, buffer.gap_start.to_string().as_str());

    stdscr.mvaddstr(10, 10, "ge");
    stdscr.mvaddstr(10, 13, buffer.gap_end.to_string().as_str());
}
