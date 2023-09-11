#![allow(non_snake_case)]

#[macro_use]
extern crate log;

use dioxus::prelude::*;
use dioxus_desktop::{Config, LogicalSize, WindowBuilder};

use facilities::setup_logger;

use gui::{
    components::{Header, Home, Loading, Settings},
    ViewName,
};

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

#[derive(Props, PartialEq, Debug)]
pub struct MainWindowProps {}

fn MainWindow(cx: Scope<MainWindowProps>) -> Element {
    trace!("MainWindow");

    let view_name = use_state(&cx, || ViewName::Home);

    cx.render(rsx! {
        link { href:"https://fonts.googleapis.com/css?family=Signika+Negative:300,400&display=swap", rel:"stylesheet", }
        style { include_str!("../../gui/waviy.css") }

        Header {
            title: "Dioxus Demo",
            view_name: view_name,
        }

        match *view_name.current() {
            ViewName::Loading => rsx! {
                Loading {
                    view_name: view_name
                }
            },
            ViewName::Home => rsx! {
                Home {}
            },
            ViewName::Settings => rsx! {
                Settings {}
            },
            _ => rsx! {
                div{}
            }
        }
    })
}
