use super::{
    new_err,
    profile::Profile,
    consts::LL_CONFIG,
    error::{
        LLResult,
        LLError
    },
    format::Format
};
use serde::Deserialize;
use std::{
    fs,
    path::PathBuf
};
use clap::{
    self,
    load_yaml,
    crate_version
};
use dirs;
use toml;

#[derive(Deserialize, Debug)]
struct ParseConfig {
    settings: Option<ParseSettings>,
    profile: Option<Profile>,
}

#[derive(Deserialize, Debug)]
struct ParseSettings {
    output_path: Option<String>,
    watch_path: Option<String>, // If set, enable watch mode
    format: Option<String> // If set, extract relevant files and place them in output_path
}

#[derive(Debug, Clone)]
pub struct Settings {
    pub output_path: String,
    pub watch_path: Option<String>,
    pub format: Format
}

#[derive(Debug, Clone)]
pub struct Cli {
    pub input: String,
    pub generate_config: (bool, bool), // (generate config?, save to users config dir?)
    pub treat_input_as_id: bool
}

#[derive(Debug, Clone)]
pub struct Config {
    pub settings: Settings,
    pub profile: Profile,
    pub cli: Cli
}

impl Config {

    pub fn load() -> Self {

        let conf: Self;

        let yml = load_yaml!("../../cli.yml");
        let matches = clap::App::from(yml).version(crate_version!()).get_matches();

        let internal = Self::try_from_fs(matches.value_of("config"));

        // This needs to be refactored
        if internal.is_ok() {

            let int = internal.unwrap();

            let settings: Settings = match int.settings {
                Some(s) => {
                    Settings {
                        output_path: match s.output_path {
                            Some (v) => String::from(v),
                            None => Self::default().settings.output_path
                        },
                        watch_path: match s.watch_path {
                            Some(v) => Some(String::from(v)),
                            None => None
                        },
                        format: match s.format {
                            Some(v) => Format::from(v),
                            None => Self::default().settings.format
                        },
                    }
                },
                None => Self::default().settings
            };

            let profile: Profile = match int.profile {
                Some(p) => p,
                None => {
                    match matches.is_present("generate") {
                        true => Profile::new("", ""),
                        false => {
                            Profile::prompt()
                        }
                    }
                }
            };

            conf = Self {
                settings: Settings {
                    output_path: match matches.value_of("output") {
                        Some(v) => String::from(v),
                        None => settings.output_path
                    },
                    watch_path: match matches.value_of("watch") {
                        Some(v) => Some(String::from(v)),
                        None => settings.watch_path
                    },
                    format: match matches.value_of("format") {
                        Some(v) => Format::from(v),
                        None => settings.format
                    }
                },
                cli: Cli {
                    input: match matches.value_of("INPUT") {
                        Some(v) => String::from(v),
                        None => Self::default().cli.input
                    },
                    generate_config: (matches.is_present("generate"), matches.is_present("home_dir")),
                    treat_input_as_id: matches.is_present("id")
                },
                profile: profile,
            }

        } else {

            conf = Self {
                settings: Settings {
                    output_path: match matches.value_of("output") {
                        Some(v) => String::from(v),
                        None => Self::default().settings.output_path
                    },
                    watch_path: match matches.value_of("watch") {
                        Some(v) => Some(String::from(v)),
                        None => Self::default().settings.watch_path
                    },
                    format: match matches.value_of("format") {
                        Some(v) => Format::from(v),
                        None => Self::default().settings.format
                    },
                },
                cli: Cli {
                    input: match matches.value_of("INPUT") {
                        Some(v) => String::from(v),
                        None => Self::default().cli.input
                    },
                    generate_config: (matches.is_present("generate"), matches.is_present("home_dir")),
                    treat_input_as_id: matches.is_present("id")
                },
                profile: match matches.is_present("generate") {
                    true => Profile::new("", ""),
                    // false => Profile::prompt()
                    false => Profile::new("", "")
                }
            };

        }

        #[cfg(debug_assertions)]
        {
            println!("-- Debug info from {file}#{line} --", file = std::file!(), line = std::line!());
            println!("{:#?}", conf);
        }

        conf
    }

    fn try_from_fs(path_input: Option<&str>) -> LLResult<ParseConfig> {

        let path = match path_input {
            Some(p) => p,
            None => LL_CONFIG
        };

        let mut conf: Option<ParseConfig> = None;

        if PathBuf::from(path).exists() {
            let data = fs::read(path)?;
            conf = Some(toml::from_slice(&data)?);
        }

        // Don't bother checking global config is local is already set.
        if conf.is_none() {

            // Check home dir for LL Config.
            let home_path = match dirs::config_dir() {
                Some(hp) => Some(hp.join(LL_CONFIG)),
                None => None
            };

            if home_path.is_some() {
                let data = fs::read(home_path.unwrap())?;
                conf = Some(toml::from_slice(&data)?);
            }

        }

        match conf {
            Some(c) => Ok(c),
            None => Err(new_err!(format!("{} not found", LL_CONFIG)))
        }

    }

    pub fn generate(global: &bool, input: &String) -> LLResult<String> {

        let path = match global {
            true => {
                match dirs::config_dir() {
                    Some(p) => p.join(LL_CONFIG),
                    None => {
                        eprintln!("Global config path could not be determined, saving to working directory");
                        PathBuf::from(LL_CONFIG)
                    }
                }
            },
            false => {
                PathBuf::from(match input.trim().is_empty() {
                    true => LL_CONFIG,
                    false => input
                })
            }
        };

        if path.clone().exists() {

            return Err(new_err!(format!("{} already exists", path.to_str().unwrap())));

        }

        match fs::write(&path, include_str!("../../LibraryLoader.example.toml")) {
            Ok(_) => {
                let path_as_string = String::from(path.to_str().unwrap());
                Ok(path_as_string)
            },
            Err(e) => Err(new_err!(e))
        }

    }

}

impl Default for Config {

    fn default() -> Self {

        let profile = Profile::new("", "");

        Self {
            settings: Settings {
                output_path: String::from("download"),
                watch_path: None,
                format: Format::from("zip")
            },
            profile: profile,
            cli: Cli {
                input: String::new(),
                generate_config: (false, false),
                treat_input_as_id: false
            }
        }

    }

}
