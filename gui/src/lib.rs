mod common;
mod header;
mod home;
mod loading;
mod settings;

pub use crate::common::ViewName;

pub mod components {
    pub use crate::header::Header;
    pub use crate::home::Home;
    pub use crate::loading::Loading;
    pub use crate::settings::Settings;
}
