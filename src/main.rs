#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
mod parse;
mod viewer;
// TODO: figure out more idiomatic way of structuring modules in
//       another file
pub use crate::parse::parsing::{parse_png, uncompress};
pub use crate::viewer::viewing::{default_settings, Viewer};

use iced::Application;
use std::path::Path;

fn main() {
    let mut unarchived = uncompress(Path::new("foo123.cbz"));
    let comic = parse_png(&mut unarchived);
    Viewer::run(default_settings(comic)).unwrap();
}
