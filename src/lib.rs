#[warn(unused_imports)]
pub use std::fs::read_to_string;
pub use std::io::{stdout, Write};
pub use termimad::crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode::*, KeyEvent},
    queue,
    style::Color::*,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
pub use termimad::*;
pub mod reader;
