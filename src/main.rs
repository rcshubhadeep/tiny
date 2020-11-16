#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod terminal;

use editor::Editor;
pub use terminal::Terminal;
use editor::Position;

use std::{panic, thread};
use std::time::Duration;

fn main() {
    let result = panic::catch_unwind(||{
        Editor::default().run();
    });

    if result.is_err() {
        thread::sleep(Duration::from_secs(3));
    }
}
