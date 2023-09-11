#![allow(non_snake_case)]

#[macro_use]
extern crate log;

use dioxus_desktop::{Config, LogicalSize, WindowBuilder};

use gui::{MainWindow, MainWindowProps};

use facilities::setup_logger;

fn main() {
    setup_logger().expect("Failed to setup logger");

    let props = MainWindowProps {};

    let config = Config::new().with_window(
        WindowBuilder::default()
            .with_title("Dioxus Demo")
            .with_inner_size(LogicalSize::new(640, 520)),
    );

    dioxus_desktop::launch_with_props(MainWindow, props, config);
}
