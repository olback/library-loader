use gtk::Builder;

mod login;
mod watch;
mod output;
mod configuration;
mod updates;

pub struct Notebook {
    pub login: login::Login,
    pub watch: watch::Watch,
    pub output: output::Output,
    pub configuration: configuration::Configuration,
    pub updates: updates::Updates
}

impl Notebook {

    pub fn build(builder: &Builder) -> Self {

        Self {
            login: login::Login::build(&builder),
            watch: watch::Watch::build(&builder),
            output: output::Output::build(&builder),
            configuration: configuration::Configuration::build(&builder),
            updates: updates::Updates::build(&builder)
        }

    }

}
