use dioxus::prelude::*;

use crate::theme::*;

#[allow(non_snake_case)]
pub fn Settings(cx: Scope) -> Element {
    let theme_state = use_shared_state::<ThemeConfig>(cx).unwrap();
    let _theme = theme_state.read();

    cx.render(rsx! {
        p {
            "Settings"
        }
    })
}
