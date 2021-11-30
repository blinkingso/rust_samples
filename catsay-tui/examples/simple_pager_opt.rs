extern crate ncurses;
extern crate structopt;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process::exit;

use ncurses::*;
use structopt::*;

#[derive(Debug, StructOpt)]
pub struct Options {
    /// File to Read or Cat
    #[structopt(short = "f", long = "file")]
    pub file: PathBuf,
}

fn prompt() -> bool {
    addstr("<- Press Any Key to Continue ->");
    // press a key;
    let ch = getch();
    if let Some(c) = std::char::from_u32(ch as u32) {
        return c == 'q';
    }

    false
}

fn open_file() -> File {
    let options: Options = Options::from_args();
    let path = options.file;
    let reader = File::open(path.as_path());
    reader
        .ok()
        .expect(format!("Unable to open file: {:?}", path.as_path()).as_ref())
}

fn main() {
    let reader = open_file().bytes();

    initscr();
    keypad(stdscr(), true);
    noecho();

    // Get the screen bounds;
    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    for ch in reader {
        if ch.is_err() {
            break;
        }
        let ch = ch.unwrap();

        /*Get the current position on the screen.*/
        let mut cur_x = 0;
        let mut cur_y = 0;
        getyx(stdscr(), &mut cur_y, &mut cur_x);

        if cur_y == (max_y - 2) {
            /*Status bar at the bottom.*/
            if !prompt() {
                /*Once a kye is pressed, clear the screen and continue.*/
                clear();
                mv(0, 0);
            } else {
                clear();
                mv(max_y - 1, 0);
                endwin();
                return;
            }
        }

        addch(ch as chtype);
    }

    /*Terminate ncurses.*/
    mv(max_y - 1, 0);
    prompt();
    endwin();
}
