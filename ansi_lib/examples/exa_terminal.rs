//! examples for terminal
extern crate ansi_term;
extern crate terminal;

use ansi_term::Colour::{Blue, Red};
use std::borrow::Cow;
use terminal::{error::Result, Action, Clear, Color, Retrieved, Value};

fn main() -> Result<()> {
    let terminal = terminal::stdout();

    // perform an single action
    terminal.act(Action::ClearTerminal(Clear::All))?;
    println!("{}", Blue.italic().paint("clear ok!"));

    // batch multiple actions.
    for i in 0..100 {
        terminal.batch(Action::MoveCursorTo(0, i))?;
        terminal.batch(Action::SetBackgroundColor(Color::Blue));
    }

    // execute batch.
    terminal.flush_batch()?;

    // get an terminal value.
    if let Retrieved::TerminalSize(x, y) = terminal.get(Value::TerminalSize)? {
        println!(
            "x: {}, y: {}",
            Red.italic().paint(Cow::from(x.to_string())),
            Blue.bold().paint(Cow::from(y.to_string()))
        );
    }

    Ok(())
}
