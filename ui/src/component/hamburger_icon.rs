use dioxus::prelude::*;

use crate::theme::*;

const JS_SCRIPT: &str = r#"
    const toggHamburgerIcon = () => {
        const navIcon = document.getElementById("nav-icon");
        navIcon.classList.toggle("open");
    };

    const toggleSlidingMenu = () => {
        const modal = document.getElementById("sliding-menu");
        modal.style.display = modal.style.display === "block" ? "none" : "block";
    };

    document
        .getElementById("nav-icon")
        .addEventListener("click", function () {
            toggHamburgerIcon();
            toggleSlidingMenu();
        });

    window.addEventListener("click", function (event) {
        const targetId = event.target.id;
        const modalOverlay = targetId === "sliding-menu";
        const anyMenuItem = targetId.includes("-item");

        if (modalOverlay || anyMenuItem) {
            toggleSlidingMenu();
            toggHamburgerIcon();
        }
    });
"#;

#[allow(non_snake_case)]
pub fn HamburgerIcon(cx: Scope) -> Element {
    let theme_state = use_shared_state::<ThemeConfig>(cx).unwrap();
    let theme = theme_state.read();

    cx.render(rsx! {
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
        script {
            "{JS_SCRIPT}"
        }
    })
}
