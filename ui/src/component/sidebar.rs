use dioxus::prelude::*;

use crate::theme::*;
use crate::view::ViewName;

#[derive(Props, PartialEq, Debug)]
pub struct SidebarProps {
    pub hidden: bool,
}

#[allow(non_snake_case)]
pub fn Sidebar(cx: Scope<SidebarProps>) -> Element {
    let theme_state = use_shared_state::<ThemeConfig>(cx).unwrap();
    let theme = theme_state.read();

    let view_state = use_shared_state::<ViewName>(cx).unwrap();

    let display_value = if cx.props.hidden { "none" } else { "block" };

    cx.render(rsx! {
        div {
            id: "sidebar-menu",
            class: "modal-overlay",
            style: "display: {display_value};",

            div {
                class: "modal-content",
                style: "background-color: {theme.back.dark}; color: {theme.font.light};",

                ul {
                    Item {
                        src: "{theme.img.about}",
                        text: "About",
                        onclick: move |_| {
                            let url = "https://dioxuslabs.com/"; // TODO: Use config
                            if let Err(e) = webbrowser::open(url) {
                                error!("Failed to open {} via browser: {}", url, e);
                            }
                        },
                    }
                    Item {
                        src: "{theme.img.settings}",
                        text: "Settings",
                        onclick: move |_| {
                            *view_state.write() = ViewName::Settings;
                        },
                    }
                    Item {
                        src: "{theme.img.quit}",
                        text: "Quit",
                        onclick: move |_| {
                            let window = dioxus_desktop::use_window(cx);
                            window.close()
                        },
                    }
                }
            }
        }
    })
}

#[derive(Props)]
struct ItemProps<'a> {
    src: &'a str,
    text: &'a str,
    onclick: EventHandler<'a>,
}

#[allow(non_snake_case)]
fn Item<'a>(cx: Scope<'a, ItemProps<'a>>) -> Element {
    let id = format!("{}-item", cx.props.text.to_lowercase().replace(" ", "-"));

    cx.render(rsx! {
        li {
            id: "{id}",

            onclick: move |_| {
                cx.props.onclick.call(());
            },

            img {
                style: "transform: scaleX(-1);",
                src: "{cx.props.src}",
            }

            "{cx.props.text}"
        }
    })
}
