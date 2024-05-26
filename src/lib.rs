#![warn(clippy::all, rust_2018_idioms)]

pub use open;

mod app;
pub use app::Fexplorer;

pub mod entries;
pub mod explorer;
pub mod search;
