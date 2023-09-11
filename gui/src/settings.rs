use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Settings(cx: Scope) -> Element {
    trace!("Settings");

    cx.render(rsx! {
        p {
            "Settings"
        }
    })
}
