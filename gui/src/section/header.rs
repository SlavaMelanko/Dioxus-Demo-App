use dioxus::prelude::*;

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

            window.onclick = function (event) {
                const modal = document.getElementById("sidebar-menu");
                if (event.target == modal) {
                    toggleSidebar();
                    toggleNavIcon();
                }
            };
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
