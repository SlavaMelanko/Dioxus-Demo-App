#![allow(non_snake_case)]

#[macro_use]
extern crate log;

use dioxus_desktop::{Config, LogicalSize, WindowBuilder};

fn main() {
    facilities::log::init().expect("Failed to setup logger");

    let props = gui::MainWindowProps {};

    let config = Config::new().with_window(
        WindowBuilder::default()
            .with_title("Dioxus Demo")
            .with_inner_size(LogicalSize::new(640, 520)),
    );

    dioxus_desktop::launch_with_props(gui::MainWindow, props, config);
}
