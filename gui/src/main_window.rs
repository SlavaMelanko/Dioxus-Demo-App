#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::common::ViewName;

use crate::{header::Header, home::Home, loading::Loading, settings::Settings};

#[derive(Props, PartialEq, Debug)]
pub struct MainWindowProps {}

pub fn MainWindow(cx: Scope<MainWindowProps>) -> Element {
    trace!("MainWindow");

    let view_name = use_state(&cx, || ViewName::Loading);

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
