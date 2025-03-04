// #![warn(clippy::all, clippy::pedantic, clippy::nursery)]

extern crate csv;
extern crate itertools;
extern crate iui;
extern crate serde_derive;
extern crate simple_excel_writer as excel;
extern crate strum;
extern crate strum_macros;

pub mod builder;
pub mod html_exporter;
pub mod spreadsheet_exporter;
pub mod ui;

#[cfg(test)]
pub mod tests;

use builder::*;
#[allow(unused_imports)]
use std::path::Path;
use ui::*;

use std::cell::RefCell;
use std::rc::Rc;

// TODO: Add more teams
// TODO: Warn user when teams would overflow
// TODO: Update people values based on their skill

fn main() {
    let tb = Rc::new(RefCell::new(TeamBuilder::new()));
    init_ui(&tb);
}
