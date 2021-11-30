extern crate ansi_term;
extern crate terminal;

use terminal::error::Result;

fn main() -> Result<()> {
    let terminal = terminal::stdout();

    Ok(())
}
