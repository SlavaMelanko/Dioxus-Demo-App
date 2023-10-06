use dioxus::prelude::*;

use crate::{theme::*, view::ViewName};

#[derive(Debug, PartialEq)]
enum LoadingStage {
    CheckingInternetConnection,
    ValidatingBinaries,
    CheckingForUpdates,
    StartingTheApp,
    Done,
}

impl ToString for LoadingStage {
    fn to_string(&self) -> String {
        match self {
            LoadingStage::CheckingInternetConnection => {
                "Checking internet connection...".to_string()
            }
            LoadingStage::ValidatingBinaries => "Validating binaries...".to_string(),
            LoadingStage::CheckingForUpdates => "Checking for updates...".to_string(),
            LoadingStage::StartingTheApp => "Starting the app...".to_string(),
            LoadingStage::Done => "Done!".to_string(),
        }
    }
}

#[allow(non_snake_case)]
pub fn Loading(cx: Scope) -> Element {
    let theme_state = use_shared_state::<ThemeConfig>(cx).unwrap();
    let theme = theme_state.read();

    let view_state = use_shared_state::<ViewName>(cx).unwrap();

    let loading_stage = use_state(&cx, || LoadingStage::CheckingInternetConnection);
    trace!("Loading: {:?}", *loading_stage.get());

    use_future(cx, (loading_stage,), |(loading_stage,)| async move {
        let mut duration: u64 = 0;
        let next: LoadingStage = match *loading_stage.get() {
            LoadingStage::CheckingInternetConnection => {
                duration = 1000;
                LoadingStage::ValidatingBinaries
            }
            LoadingStage::ValidatingBinaries => {
                duration = 750;
                LoadingStage::CheckingForUpdates
            }
            LoadingStage::CheckingForUpdates => {
                duration = 1500;
                LoadingStage::StartingTheApp
            }
            LoadingStage::StartingTheApp => {
                duration = 500;
                LoadingStage::Done
            }
            LoadingStage::Done => LoadingStage::Done,
        };

        tokio::time::sleep(tokio::time::Duration::from_millis(duration)).await;
        loading_stage.set(next);
    });

    // TODO: Doesn't work inside the future closure
    if *loading_stage.get() == LoadingStage::Done {
        *view_state.write() = ViewName::Home;
    }

    cx.render(rsx! {
        div {
            class: "loading",
            style: "background-color: {theme.back.light}; color: {theme.font.light};",

            div {
                class: "waviy",
                style: "color: {theme.back.mid};",
                get_loading_text().chars().enumerate().map(|(i, l)| rsx!{
                    span { style: "--i:{i};", "{l}"}
                }),
            }
            // To see an animation, we need to render the text using a new element each time
            match loading_stage.get() {
                LoadingStage::CheckingInternetConnection | LoadingStage::CheckingForUpdates => {
                    rsx!(div {
                        class: "line",
                        a {
                            class: "lineUp",
                            style: "color: {theme_state.read().font.light};",
                            loading_stage.get().to_string()
                        }
                    })
                }
                LoadingStage::ValidatingBinaries | LoadingStage::StartingTheApp => {
                    rsx!(div {
                        class: "line",
                        a {
                            class: "lineUp",
                            style: "color: {theme_state.read().font.light};",
                            loading_stage.get().to_string()
                        }
                    })
                }
                LoadingStage::Done => { // TODO: No need?
                    rsx!(div {
                        class: "line",
                        a {
                            class: "lineUp",
                            style: "color: {theme_state.read().font.light};",
                            ""
                        }
                    })
                }
            }
        }
    })
}

fn get_loading_text() -> String {
    "LOADING...".to_string()
}
