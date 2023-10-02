use dioxus::prelude::*;

use crate::component::Title;
use crate::theme::*;

const NAV_ICON_JS_SCRIPT: &str = r#"
    const toggleNavIcon = () => {
        const navIcon = document.getElementById("nav-icon");
        navIcon.classList.toggle("open");
    };

    const toggleSidebar = () => {
        const modal = document.getElementById("sidebar-menu");
        modal.style.display = modal.style.display === "block" ? "none" : "block";
    };

    document
        .getElementById("nav-icon")
        .addEventListener("click", function () {
            toggleNavIcon();
            toggleSidebar();
        });

    window.addEventListener("click", function (event) {
        const targetId = event.target.id;
        const modalOverlay = targetId === "sidebar-menu";
        const anyMenuItem = targetId.includes("-item");

        if (modalOverlay || anyMenuItem) {
            toggleSidebar();
            toggleNavIcon();
        }
    });
    "#;

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

            div {
                id: "theme-icon",

                onclick: |_| {
                    let id = theme_state.read().id;
                    *theme_state.write() = match id {
                        Id::Dark => ThemeConfig::make_light_theme_config(),
                        Id::Light => ThemeConfig::make_dark_theme_config(),
                    };
                },

                img { src: "{theme.img.theme}" },
            }
            div {
                id: "nav-icon",

                span {
                    style: "background-color: {theme.font.dark};",
                }
                span {
                    style: "background-color: {theme.font.dark};",
                }
                span {
                    style: "background-color: {theme.font.dark};",
                }
            }
        }
        script {
            "{NAV_ICON_JS_SCRIPT}"
        }
    })
}
