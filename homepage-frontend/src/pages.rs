//! Pages

// Modules
mod about_me;
mod cv;
mod home;
mod not_found;
mod projects;

// Exports
pub use self::{about_me::AboutMe, cv::CV, home::Home, not_found::NotFound, projects::Projects};
