mod config;
mod consts;
mod cse;
mod epw;
mod error;
mod format;
mod logger;
mod updates;
mod utils;
mod watcher;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const GIT_DESCRIBE: &str = include_str!("../git.txt");

pub use {
    config::{profile::Profile, Config, Format},
    consts::LL_CONFIG,
    error::{Error, Result},
    format::ECAD,
    logger::{ConsoleLogger, Logger},
    updates::check as check_updates,
    updates::{ClientKind, UpdateInfo},
    watcher::Watcher,
};
