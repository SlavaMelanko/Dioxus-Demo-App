#[macro_use]
extern crate log;

mod chart;
mod main_window;
mod theme;

pub(crate) mod section;
pub(crate) mod view;
pub(crate) mod component;

pub use crate::main_window::{MainWindow, MainWindowProps};
