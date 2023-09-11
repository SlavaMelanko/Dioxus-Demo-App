#![allow(non_snake_case)]

use dioxus::prelude::*;

pub fn Home(cx: Scope) -> Element {
    trace!("Home");

    cx.render(rsx!(p {
        "Home"
    }))
}
