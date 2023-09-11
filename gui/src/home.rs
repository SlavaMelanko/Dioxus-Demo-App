#![allow(non_snake_case)]

use dioxus::prelude::*;

pub(crate) fn Home(cx: Scope) -> Element {
    trace!("Home");

    cx.render(rsx!(p {
        "Home"
    }))
}
