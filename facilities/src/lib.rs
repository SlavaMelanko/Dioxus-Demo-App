extern crate fern;

mod logger;

pub mod log {
    pub use crate::logger::setup_logger as init;
}
