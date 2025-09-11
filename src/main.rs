mod app;
mod notes;
mod ui;

use std::{fs, io};
use notes::get_notes_dir;
use ui::run_tui;




fn main() -> Result<(), io::Error> {
    let directory = get_notes_dir();
    fs::create_dir_all(&directory)?;
    run_tui(&directory)?;
    Ok(())
}
