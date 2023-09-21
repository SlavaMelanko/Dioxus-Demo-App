use dioxus::prelude::*;

use crate::theme::*;

#[derive(Props)]
pub struct FooterProps<'a> {
    copyright: &'a str,
}

#[allow(non_snake_case)]
pub fn Footer<'a>(cx: Scope<'a, FooterProps<'a>>) -> Element {
    let theme = use_shared_state::<Box<dyn Theme>>(cx).unwrap().read();

    cx.render(rsx! {
        div {
            class: "footer",
            style: "background-color: {theme.back_dark()}; color: {theme.text_light()};",

            "{cx.props.copyright}"
        }
    })
}
