extern crate ncurses;

use ncurses::*;

fn main() {
    initscr();

    addstr("Hello, world!");

    /*Update the screen*/
    refresh();

    /* Wait for a key press */
    getch();

    /* Terminate ncurses. */
    endwin();
}
