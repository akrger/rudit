extern crate rudit;
extern crate pancurses;

use pancurses::*;
use rudit::gapbuffer::GapBuffer;

fn main() {
    let mut buffer = GapBuffer::new(30);

    let mut stdscr = initscr();

    stdscr.keypad(true);
    noecho();
    nl();
    cbreak();
    let mut cursor_x: usize = 0;
    let mut cursor_y: usize = 0;
    let mut lines: usize = 0;
    loop {
        // stdscr.clear();
        draw_buffer(buffer.buffer.clone(), &mut stdscr);
        draw_info(&mut stdscr, &buffer, cursor_x as i32, cursor_y as i32);
        move_cursor(cursor_x as i32, cursor_y as i32, &mut stdscr);
        stdscr.refresh();

        match stdscr.getch() {
            Some(Input::Character(c)) => {
                if c == 'q' {
                    endwin();
                    println!("{}", buffer);
                    println!("{:?}", buffer.buffer);
                    break;
                }
                if c == '\n' {
                    buffer.insert(cursor_x, '\n');
                    cursor_y += 1;
                    lines += 1;
                } else {
                    buffer.insert(cursor_x, c);
                    cursor_x += 1;

                }
            }
            Some(Input::KeyRight) => {
                if cursor_x < buffer.len() {
                    cursor_x += 1;
                }
            }
            Some(Input::KeyLeft) => {
                if cursor_x > 0 {
                    cursor_x -= 1;
                }
            }
            Some(Input::KeyDC) => {
                if cursor_x > 0 {
                    cursor_x -= 1;
                    buffer.delete(cursor_x);
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
        } else if c == '\0' {
        } else {
            stdscr.mvaddch(y, x, c);
            x += 1;
        }
    }
}
fn move_cursor(x: i32, y: i32, stdscr: &mut Window) {
    stdscr.mv(y, x);
    stdscr.refresh();
}
fn draw_info(stdscr: &mut Window, buffer: &GapBuffer, cursor_x: i32, cursor_y: i32) {
    stdscr.mvaddstr(10, 0, "cx");
    stdscr.mvaddstr(10, 3, cursor_x.to_string().as_str());

    stdscr.mvaddstr(10, 5, "gs");
    stdscr.mvaddstr(10, 8, buffer.gap_start.to_string().as_str());

    stdscr.mvaddstr(10, 10, "ge");
    stdscr.mvaddstr(10, 13, buffer.gap_end.to_string().as_str());

    stdscr.mvaddstr(10, 20, cursor_x.to_string().as_str());
    stdscr.mvaddstr(10, 24, (cursor_y + 1).to_string().as_str());
}
