use gtk::{Builder, ApplicationWindow};

mod account;
mod watch;
mod output;
mod configuration;
mod updates;

pub struct Notebook {
    pub account: account::Account,
    pub watch: watch::Watch,
    pub output: output::Output,
    pub configuration: configuration::Configuration,
    pub updates: updates::Updates
}

impl Notebook {

    pub fn build(builder: &Builder, main_window: &ApplicationWindow) -> Self {

        Self {
            account: account::Account::build(&builder),
            watch: watch::Watch::build(&builder, &main_window),
            output: output::Output::build(&builder),
            configuration: configuration::Configuration::build(&builder),
            updates: updates::Updates::build(&builder)
        }

    }

}
