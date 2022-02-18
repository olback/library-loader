use {
    ll_core::{Config, ConsoleLogger, Error, Watcher},
    std::path::PathBuf,
};

fn main() -> ll_core::Result<()> {
    let app_yaml = clap::load_yaml!("../cli.yml");
    let app = clap::App::from(app_yaml)
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .get_matches();

    #[cfg(not(debug_assertions))]
    let update_handle = std::thread::spawn(|| {
        match ll_core::check_updates(env!("CARGO_PKG_VERSION"), ll_core::ClientKind::CLI) {
            Ok(None) => {} // Latest version
            Ok(Some(ui)) => {
                println!("=========================================================");
                println!("New update {} available!", ui.remote);
                println!("Currently installed: {}", ui.local);
                println!("{}", ui.url);
                println!("=========================================================");
            }
            Err(e) => {
                eprintln!("Error checking for updates: {:?}", e);
            }
        }
    });

    let config_path = match if app.is_present("global_config") {
        Config::default_path()
    } else {
        app.value_of("config")
            .map(PathBuf::from)
            .or(Config::get_path()?)
    } {
        Some(path) => path,
        None => {
            eprintln!("No config Library Loader config exists.");
            eprintln!("You can generate a global config:");
            eprintln!("\t{} -g -u", env!("CARGO_PKG_NAME"));
            eprintln!("If you want go generate a local config:");
            eprintln!("\t{} -g -c {}", env!("CARGO_PKG_NAME"), ll_core::LL_CONFIG);
            return Err(Error::NoConfig);
        }
    };

    if app.is_present("generate") {
        if config_path.exists() && !app.is_present("overwrite") {
            eprintln!("File {:?} already exists, quitting...", config_path);
            return Err(Error::WouldOverwrite);
        } else {
            println!("Writing default config to {:?}", config_path);
            let config = Config::default();
            config.save(Some(config_path))?;
            return Ok(());
        }
    }

    println!("Using config at {:?}", config_path);
    let mut config = Config::read(Some(config_path))?;

    if let Some(watch_path) = app.value_of("watch") {
        config.settings.watch_path = watch_path.into();
    }

    if config.profile.username.is_empty() || config.profile.password.is_empty() {
        eprintln!("You're not logged in.");
        eprintln!("Update username and password in config");
        return Err(Error::NotLoggedIn);
    }

    if config.formats.is_empty() {
        eprintln!("No formats specified in config.");
        eprintln!("This is not an error but the program is useless");
        eprintln!("without any formats specified.");
    }

    let mut watcher = Watcher::new(config, vec![ConsoleLogger::new()])?;
    watcher.start()?;

    println!("Stop by pressing <Enter>");
    std::io::stdin().read_line(&mut String::with_capacity(1))?;

    watcher.stop();

    #[cfg(not(debug_assertions))]
    drop(update_handle.join());

    Ok(())
}
