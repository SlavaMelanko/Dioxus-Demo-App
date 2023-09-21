#![allow(non_snake_case)]

extern crate log;

use dioxus_desktop::{Config, LogicalSize, WindowBuilder};

fn main() {
    facilities::log::init().expect("Failed to setup logger");

    let props = gui::MainWindowProps {};

    let config = Config::new().with_window(
        WindowBuilder::default()
            .with_title("Dioxus Demo") // TODO: Use config
            .with_inner_size(LogicalSize::new(640, 520)), // TODO: Use config too
    );

    dioxus_desktop::launch_with_props(gui::MainWindow, props, config);
}
