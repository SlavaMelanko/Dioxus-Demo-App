#![allow(non_snake_case)]

use dioxus::prelude::*;

pub fn Settings(cx: Scope) -> Element {
    trace!("Settings");

    cx.render(rsx!(p {
        "Settings"
    }))
}
