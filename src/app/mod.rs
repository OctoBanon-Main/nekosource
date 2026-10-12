pub mod error;
pub mod git;
pub mod gpg;
mod language;
mod state;
mod ui;

pub use ui::NekoView as NekoApp;
