use dioxus::prelude::*;

use crate::section::{Body, Footer, Header};
use crate::{theme::*, view::*};

#[derive(Props, PartialEq, Debug)]
pub struct MainWindowProps {}

#[allow(non_snake_case)]
pub fn MainWindow(cx: Scope<MainWindowProps>) -> Element {
    use_shared_state_provider(cx, get_default_theme_provider);
    use_shared_state_provider(cx, get_default_view_name);

    cx.render(rsx! {
        link { href:"https://fonts.googleapis.com/css?family=Signika+Negative:300,400&display=swap", rel:"stylesheet", }
        style { include_str!("../../gui/style.css") }

        Header {
            title: "Dioxus Demo", // TODO: Use config
        }

        Body {}

        Footer {
            copyright: "Slava Melanko © 2023", // TODO: Use config
        }
    })
}
