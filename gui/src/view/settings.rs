use dioxus::prelude::*;

use crate::theme::*;

#[allow(non_snake_case)]
pub fn Settings(cx: Scope) -> Element {
    let _theme = use_shared_state::<Box<dyn Theme>>(cx).unwrap().read();

    cx.render(rsx! {
        p {
            "Settings"
        }
    })
}
