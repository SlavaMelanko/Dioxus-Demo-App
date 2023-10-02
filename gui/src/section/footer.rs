use dioxus::prelude::*;

use crate::theme::*;

#[derive(Props)]
pub struct FooterProps<'a> {
    copyright: &'a str,
}

#[allow(non_snake_case)]
pub fn Footer<'a>(cx: Scope<'a, FooterProps<'a>>) -> Element {
    let theme_state = use_shared_state::<ThemeConfig>(cx).unwrap();
    let theme = theme_state.read();

    cx.render(rsx! {
        div {
            class: "footer",
            style: "background-color: {theme.back.dark}; color: {theme.font.dark};",

            "{cx.props.copyright}"
        }
    })
}
