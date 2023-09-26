use dioxus::prelude::*;

use crate::theme::*;
use crate::view::{Home, Loading, Settings, ViewName};

#[allow(non_snake_case)]
pub fn Body(cx: Scope) -> Element {
    let theme = use_shared_state::<Box<dyn Theme>>(cx).unwrap().read();
    let view_shared_state = use_shared_state::<ViewName>(cx).unwrap();

    let window = dioxus_desktop::use_window(cx);

    cx.render(rsx! {
        div {
            class: "body",
            style: "background-color: {theme.back_light()}; color: {theme.text_light()};",

            match *view_shared_state.read() {
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

            div {
                id: "sidebar-menu",
                class: "modal",
                style: "display: none;",

                div {
                    class: "modal-content",
                    style: "background-color: {theme.back_dark()}; color: {theme.text_light()};",
                    div {
                        class: "modal-body",
                        ul {
                            li { "About" }
                            li {
                                img {
                                    src: "{theme.img_settings()}",
                                }

                                "Settings"
                            }
                            li {
                                onmousedown: |evt| evt.stop_propagation(),
                                onclick: move |_| window.close(),

                                img {
                                    style: "transform: scaleX(-1);",
                                    src: "{theme.img_quit()}",
                                }

                                "Quit"
                            }
                        }
                    }
                }
            }
        }
    })
}
