#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::common::ViewName;

const NAV_ICON_JS_SCRIPT: &str = r#"
    document
        .getElementById("nav-icon")
        .addEventListener("click", function () {
            this.classList.toggle("open");
        });

    document
        .getElementById("theme-icon")
        .addEventListener("click", function () {
            document.body.classList.toggle("dark-theme");
        });
    "#;

#[derive(Props)]
pub(crate) struct HeaderProps<'a> {
    title: &'a str,
    view_name: &'a UseState<ViewName>,
}

pub(crate) fn Header<'a>(cx: Scope<'a, HeaderProps<'a>>) -> Element {
    let theme_img = use_state(&cx, || "gui/img/moon-32.png".to_string());

    cx.render(rsx!(
        div {
            class: "header",
            div {
                class: "title",
                "{cx.props.title}"
            }
            div {
                id: "theme-icon",
                onclick: |_| {
                    if theme_img.get() == "gui/img/moon-32.png" {
                        theme_img.set("gui/img/sun-32.png".to_string());
                    } else {
                        theme_img.set("gui/img/moon-32.png".to_string());
                    }
                },
                img { src: "{theme_img}" },
            }
            div {
                id: "nav-icon",
                onclick: move |_| {
                    let current_view = *cx.props.view_name.get();
                    if current_view == ViewName::Home {
                        cx.props.view_name.set(ViewName::Settings);
                    } else {
                        cx.props.view_name.set(ViewName::Home);
                    }
                },
                span {}
                span {}
                span {}
            }
        }
        script {
            "{NAV_ICON_JS_SCRIPT}"
        }
    ))
}
