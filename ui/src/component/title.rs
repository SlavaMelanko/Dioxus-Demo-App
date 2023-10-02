use dioxus::prelude::*;

use crate::view::ViewName;

#[derive(Props)]
pub struct TitleProps<'a> {
    title: &'a str,
}

#[allow(non_snake_case)]
pub fn Title<'a>(cx: Scope<'a, TitleProps<'a>>) -> Element {
    let view_state = use_shared_state::<ViewName>(cx).unwrap();

    cx.render(rsx! {
        div {
            class: "title",

            onclick: move |_| {
                *view_state.write() = ViewName::Home;
            },

            "{cx.props.title}"
        }
    })
}
