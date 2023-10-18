use dioxus::prelude::*;

use crate::component::SlidingMenu;
use crate::theme::*;
use crate::view::{Home, Loading, Settings, ViewName};

#[allow(non_snake_case)]
pub fn Body(cx: Scope) -> Element {
    let theme_state = use_shared_state::<ThemeConfig>(cx).unwrap();
    let theme = theme_state.read();

    let view_state = use_shared_state::<ViewName>(cx).unwrap();

    cx.render(rsx! {
        div {
            class: "body",
            style: "background-color: {theme.back.light}; color: {theme.font.light};",

            match *view_state.read() {
                ViewName::Loading => rsx! {
                    Loading {}
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

            SlidingMenu {
                hidden: true,
            }
        }
    })
}
