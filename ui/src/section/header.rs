use dioxus::prelude::*;

use crate::component::HamburgerIcon;
use crate::component::ThemeToggle;
use crate::component::Title;
use crate::theme::*;

#[derive(Props)]
pub struct HeaderProps<'a> {
    title: &'a str,
}

#[allow(non_snake_case)]
pub fn Header<'a>(cx: Scope<'a, HeaderProps<'a>>) -> Element {
    let theme_state = use_shared_state::<ThemeConfig>(cx).unwrap();
    let theme = theme_state.read();

    cx.render(rsx! {
        div {
            class: "header",
            style: "background-color: {theme.back.dark}; color: {theme.font.light};",

            Title {
                title: cx.props.title,
            }

            ThemeToggle {}

            HamburgerIcon {}
        }
    })
}
