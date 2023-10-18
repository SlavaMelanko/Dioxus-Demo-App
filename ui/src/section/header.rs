use dioxus::prelude::*;

use serde_json::json;

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

const THEME_ICON_JS_SCRIPT: &str = r#"
    let themeConfig = {
        value: "dark",
    };

    const setPreference = () => {
        document.firstElementChild.setAttribute("data-theme", themeConfig.value);
    };

    const handleThemeSwitchClicked = () => {
        themeConfig.value = themeConfig.value === "light" ? "dark" : "light";
        setPreference();
    };

    // now this script can find and listen for clicks on the control
    document.getElementById("theme-toggle").addEventListener("click", handleThemeSwitchClicked);

    window
        .matchMedia("(prefers-color-scheme: dark)")
        .addEventListener("change", ({ matches: isDark }) => {
            themeConfig.value = isDark ? "dark" : "light";
            setPreference();
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

    let eval_provider = use_eval(cx);
    let theme_mode = theme.id.to_string();
    use_effect(cx, (), move |_| {
        to_owned![eval_provider, theme_mode];
        async move {
            let eval = eval_provider(
                r#"
                    themeConfig = await dioxus.recv();
                    setPreference();
                "#,
            )
            .unwrap();

            eval.send(json!({"value": theme_mode})).unwrap();
        }
    });

    cx.render(rsx! {
        div {
            class: "header",
            style: "background-color: {theme.back.dark}; color: {theme.font.light};",

            Title {
                title: cx.props.title,
            }

            button {
                class: "theme-toggle",
                id: "theme-toggle",

                onclick: |_| {
                    let id = theme_state.read().id;
                    *theme_state.write() = match id {
                        Id::Dark => ThemeConfig::make_light_theme_config(),
                        Id::Light => ThemeConfig::make_dark_theme_config(),
                    };
                },

                svg {
                    class: "sun-and-moon",
                    width: "24",
                    height: "24",
                    "viewBox": "0 0 24 24",

                    mask {
                        class: "moon",
                        id: "moon-mask",

                        rect {
                            x: "0",
                            y: "0",
                            width: "100%",
                            height: "100%",
                            fill: "white",
                        }
                        circle {
                            cx: "24",
                            cy: "10",
                            r: "6",
                            fill: "black",
                        }
                    }
                    circle {
                        class: "sun",
                        cx: "12",
                        cy: "12",
                        r: "6",
                        mask: "url(#moon-mask)",
                        fill: "currentColor",
                    }
                    g {
                        class: "sun-beams",
                        stroke: "currentColor",

                        line {
                            x1: "12",
                            y1: "1",
                            x2: "12",
                            y2: "3",
                        },
                        line {
                            x1: "12",
                            y1: "21",
                            x2: "12",
                            y2: "23",
                        },
                        line {
                            x1: "4.22",
                            y1: "4.22",
                            x2: "5.64",
                            y2: "5.64",
                        },
                        line {
                            x1: "18.36",
                            y1: "18.36",
                            x2: "19.78",
                            y2: "19.78",
                        },
                        line {
                            x1: "1",
                            y1: "12",
                            x2: "3",
                            y2: "12",
                        },
                        line {
                            x1: "21",
                            y1: "12",
                            x2: "23",
                            y2: "12",
                        },
                        line {
                            x1: "4.22",
                            y1: "19.78",
                            x2: "5.64",
                            y2: "18.36",
                        },
                        line {
                            x1: "18.36",
                            y1: "5.64",
                            x2: "19.78",
                            y2: "4.22",
                        },
                    }
                }
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
        script {
            "{THEME_ICON_JS_SCRIPT}"
        }
    })
}
