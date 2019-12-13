use gtk::{Builder, ApplicationWindow};
use crate::types::AMState;

pub mod account;
pub mod watch;
pub mod output;
pub mod configuration;
pub mod updates;

#[derive(Debug)]
pub struct Notebook {
    pub account: account::Account,
    pub watch: watch::Watch,
    pub output: output::Output,
    pub configuration: configuration::Configuration,
    pub updates: updates::Updates
}

impl Notebook {

    pub fn build(builder: &Builder, main_window: &ApplicationWindow, state: &AMState) -> Self {

        Self {
            account: account::Account::build(&builder, &state),
            watch: watch::Watch::build(&builder, &main_window, &state),
            output: output::Output::build(&builder),
            configuration: configuration::Configuration::build(&builder, &state),
            updates: updates::Updates::build(&builder)
        }

    }

}
