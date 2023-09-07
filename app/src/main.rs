#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_desktop::{Config, LogicalSize, WindowBuilder};

use gui::components::Header;
use gui::components::Home;
use gui::components::Loading;
use gui::components::Settings;
use gui::ViewName;

fn main() {
    let props = AppProps {};

    let config = Config::new().with_window(
        WindowBuilder::default()
            .with_title("Dioxus Demo")
            .with_inner_size(LogicalSize::new(640, 520)),
    );

    dioxus_desktop::launch_with_props(App, props, config);
}

#[derive(Props, PartialEq, Debug)]
pub struct AppProps {}

fn App(cx: Scope<AppProps>) -> Element {
    println!("App");

    let current_view = use_state(&cx, || ViewName::Loading);

    println!("View: {:?}", current_view);

    cx.render(rsx! {
        link { href:"https://fonts.googleapis.com/css?family=Signika+Negative:300,400&display=swap", rel:"stylesheet", }
        style { include_str!("../../gui/waviy.css") }

        Header {
            title: "Dioxus Demo",
            current_view: current_view,
        }

        match *current_view.current() {
            ViewName::Loading => rsx! {
                Loading {
                    current_view: current_view
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
