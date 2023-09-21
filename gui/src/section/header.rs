use dioxus::prelude::*;

use crate::{theme::*, view::ViewName};

const NAV_ICON_JS_SCRIPT: &str = r#"
    document
        .getElementById("nav-icon")
        .addEventListener("click", function () {
            this.classList.toggle("open");
        });
    "#;

#[derive(Props)]
pub struct HeaderProps<'a> {
    title: &'a str,
}

#[allow(non_snake_case)]
pub fn Header<'a>(cx: Scope<'a, HeaderProps<'a>>) -> Element {
    let theme_shared_state = use_shared_state::<Box<dyn Theme>>(cx).unwrap();
    let theme = theme_shared_state.read();

    let view_shared_state = use_shared_state::<ViewName>(cx).unwrap();

    cx.render(rsx! {
        div {
            class: "header",
            style: "background-color: {theme.back_dark()}; color: {theme.text_light()};",

            div {
                class: "title",
                "{cx.props.title}"
            }
            div {
                id: "theme-icon",
                onclick: |_| {
                    let current_theme_id = theme_shared_state.read().id();
                    *theme_shared_state.write() = match current_theme_id {
                        Id::Dark => LightTheme::new(),
                        Id::Light => DarkTheme::new(),
                    };
                },
                img { src: "{theme.img_theme()}" },
            }
            div {
                id: "nav-icon",
                onclick: move |_| {
                    let current_view = *view_shared_state.read();
                    *view_shared_state.write() = if current_view == ViewName::Home {
                        ViewName::Settings
                    } else {
                        ViewName::Home
                    };
                },
                span {
                    style: "background-color: {theme.text_dark()};",
                }
                span {
                    style: "background-color: {theme.text_dark()};",
                }
                span {
                    style: "background-color: {theme.text_dark()};",
                }
            }
        }
        script {
            "{NAV_ICON_JS_SCRIPT}"
        }
    })
}
