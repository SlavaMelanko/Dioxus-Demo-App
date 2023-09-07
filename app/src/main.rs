#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_desktop::{Config, LogicalSize, WindowBuilder};

use gui::{
    components::{Header, Home, Loading, Settings},
    ViewName,
};

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
    println!("App"); // TODO: Use fern for logging https://docs.rs/fern/0.5.8/fern/#example-setup

    let view_name = use_state(&cx, || ViewName::Loading);
    println!("View: {:?}", view_name);

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
