use dioxus::prelude::*;

#[allow(non_snake_case)]
pub(crate) fn Home(cx: Scope) -> Element {
    trace!("Home");

    cx.render(rsx! {
        p {
            "Home"
        }
    })
}
