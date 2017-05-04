extern crate rudit;
extern crate pancurses;
use pancurses::*;
use rudit::gapbuffer::GapBuffer;
fn main() {
    let mut buffer = GapBuffer::new(10);
    buffer.insert(0, 'b');
    buffer.insert(1, 'a');
    // buffer.insert(2, 'a');
    // buffer.insert(3, 'b');
    let mut stdscr = initscr();
    stdscr.keypad(true);
    noecho();
    nl();
    cbreak();
    let mut cx = 0;
    let mut cy = 0;
    loop {
        draw_buffer(buffer.buffer.clone(), &mut stdscr);
        move_cursor(cx, cy, &mut stdscr);
        stdscr.refresh();
        match stdscr.getch() {
            Some(Input::Character(c)) => {
                if c == '\n' {
                    cy += 1;
                    cx = 0;
                    buffer.insert(cx as usize, '\n');
                    continue;
                }
                buffer.insert(cx as usize, c);
                cx += 1;
            }
            Some(Input::KeyEnter) => {
                cy += 1;
                cx = 0;
                buffer.insert(cx as usize, '\n');
            }
            Some(Input::KeyRight) => {
                cx += 1;
            }
            Some(Input::KeyLeft) => {
                if cx > 0 {
                    cx -= 1;
                }
            }

            _ => (),
        }
    }
    endwin();
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
}
